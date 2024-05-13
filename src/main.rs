mod _redis;
mod agent;
mod ball;
mod gameobject;
mod getstate;
mod pitch;
mod player;
mod position;
mod team;
mod visibleplayer;
mod window;

use agent::{make_random_actions, AgentAction};
use football::get_quad_gl;
use gameobject::GameObject;
use macroquad::{prelude::*, rand};
use std::{borrow::BorrowMut, collections::HashMap, env, hash::Hash, time::UNIX_EPOCH};

use ball::*;
use pitch::*;
use player::Player;
use position::Position;
use serde::{self, Deserialize, Serialize};
use team::*;
use uuid::Uuid;
use visibleplayer::*;
use window::*;
use std::time;

/// Football
/// This rust program makes a set of players, puts them in a pitch & simulates something similar to football,
/// more closely described as some pool-football hybrid.
/// Players can bounce of the walls & off each other, they can choose to kick the ball

#[cfg(feature = "use_redis")]
use redis;

const PHYSICS_TICK_RATE: f32 = 1.0 / 30.0; // in seconds

fn set_rand_seed() {
    let seed = time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    rand::srand(seed)
}

#[macroquad::main("Football")]
async fn main() {
    set_rand_seed();
    let game_id = Uuid::new_v4().to_string();
    let screen = ScreenSize::new();

    // set_window_size(screen.width as u32, screen.height as u32);
    let pitch = Pitch::new(&screen);

    // let's setup a bit like a school 5 a side, make all the players,
    // then assign players to teams
    let n_players: usize = env::var("n_players")
        .unwrap_or("10".to_owned())
        .parse()
        .unwrap();
    let team1_players = Team::generate_players(n_players / 2);
    let team2_players = Team::generate_players(n_players / 2);

    let team1 = Team::new(String::from("Team 1"), TeamSide::Home);
    let mut team1_players = team1.setup_team(&team1_players, &pitch);
    let team2 = Team::new(String::from("Team 2"), TeamSide::Away);
    let mut team2_players = team2.setup_team(&team2_players, &pitch);
    let mut all_players = [team1_players.clone(), team2_players.clone()].concat();

    const PHYSICS_TICK_RATE: f32 = 1.0 / 30.0; // in seconds
    let ball = Ball::new(
        ((pitch.width / 2) + pitch.x) as f32,
        ((pitch.height / 2) + pitch.y) as f32,
    );
    let mut time_accumulator: f32 = 0.0;
    // make top left player movable by arrow keys
    // good for local testing
    all_players[0].borrow_mut().to_movable();

    let score: u8 = 0;

    // let mut all_players = team1.players.clone();
    // all_players.extend(team2.players.clone());
    let mut state = GameState {
        p_frame: 0,
        uuid: game_id.clone(),
        players: all_players,
        actions: HashMap::new(),
        pitch,
        ball,
        team1,
        team2,
        score,
    };

    #[cfg(feature = "use_redis")]
    let client = _redis::setup_redis(game_id);

    state.ball.game_start_kick(PHYSICS_TICK_RATE);

    let qgl = get_quad_gl!();
    loop {
        time_accumulator += get_frame_time();

        while time_accumulator >= PHYSICS_TICK_RATE {
            #[cfg(feature = "use_redis")]
            state.log(&client);

            // todo: replace random actions with reads from redis
            state.get_random_actions();
            apply_physics(qgl, &mut state);
            time_accumulator -= PHYSICS_TICK_RATE;
        }
        let alpha: f32 = time_accumulator / PHYSICS_TICK_RATE;
        render(qgl, &mut state, alpha);
        next_frame().await
    }
}

fn apply_physics(qgl: &mut QuadGl, state: &mut GameState) {
    state.p_frame += 1;
    state.ball.object.apply_friction(PHYSICS_TICK_RATE);
    state
        .ball
        .object
        .update_position(&state.pitch, PHYSICS_TICK_RATE);

    for visibleplayer in state.players.iter_mut() {
        match state.actions.get(&visibleplayer.player.uuid) {
            None => continue,
            Some(action) => {
                if state.ball.is_kickable_by(&visibleplayer) {
                    let (x, y) = (action.x, action.y);

                    visibleplayer.borrow_mut().handle_kick_ball(
                        &mut state.ball,
                        x,
                        y,
                        PHYSICS_TICK_RATE,
                    );
                }
                visibleplayer.handle_movement(qgl, PHYSICS_TICK_RATE, &action.movement);
                visibleplayer.handle_physics(&state.pitch, PHYSICS_TICK_RATE)
            }
        }
    }
    {
        // check collisions between players
        let _radius = state.players[0].borrow_mut().object.radius;
        gameobject::arange_checks(&mut state.players, _radius);
    }

    let goal = &state.pitch.check_goal(&state.ball);
    match goal {
        Some(g) => match g {
            Goal::HOME => state.score = 1,
            Goal::AWAY => state.score = 2,
        },
        _ => {}
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
/// Contains all information about the state of the game at a given timestep
/// * p_frame: physics frame, which tick of the physics engine we've hit
/// * uuid: a String representing curent game id
/// * ball: the game `Ball`
/// * team1: the Home `Team`
/// * team2: the Away `Team`
/// * pitch: the `Pitch` being played on
/// * score: todo: fix this
///
pub struct GameState {
    p_frame: i32,
    pub uuid: String,
    #[serde(skip)]
    pub players: Vec<VisiblePlayer>,
    actions: HashMap<String, AgentAction>,
    pub ball: Ball,
    pub team1: Team,
    pub team2: Team,
    pub pitch: Pitch,
    pub score: u8,
}

impl GameState {
    fn get_random_actions(&mut self) {
        self.actions = make_random_actions(&self.players)
    }
}

fn render(qgl: &mut QuadGl, state: &mut GameState, alpha: f32) {
    clear_background(WHITE);
    draw_text(
        &format!(
            "ball x: {:?}, y: {:?}",
            state.ball.object.pos.x, state.ball.object.pos.y
        ),
        320.0,
        12.0,
        20.0,
        macroquad::color::BLACK,
    );
    render_pitch(qgl, &state.pitch);
    for visibleplayer in state.players.iter() {
        visibleplayer.render(qgl, alpha);
    }
    draw_text(
        &format!("{}", get_fps()),
        100.0,
        12.0,
        10.0,
        macroquad::color::BLACK,
    );
    state.ball.render(qgl, alpha);
    draw_text(
        &format!("Ball speed x: {}", state.ball.object.x_velocity),
        200.0,
        120.0,
        10.0,
        macroquad::color::BLACK,
    );
    draw_text(
        &format!("Ball speed y: {}", state.ball.object.y_velocity),
        200.0,
        100.0,
        10.0,
        macroquad::color::BLACK,
    );

    if state.score > 0 {
        draw_text(
            &format!("Score is {:?}", state.score),
            10.0,
            200.0,
            5.0,
            macroquad::color::GOLD,
        )
    }
}

#[cfg(feature = "use_redis")]
trait LogRedis {
    fn log(&self, client: &redis::Client) {}

    fn read_actions(&self, client: &redis::Client) {}
}

#[cfg(feature = "use_redis")]
impl LogRedis for GameState {
    fn log(&self, client: &redis::Client) {
        let mut con = client.get_connection().unwrap();
        let state_str = serde_json::to_string(&self).unwrap();
        _redis::write_redis(&mut con, &self.uuid, state_str.as_str());
    }

    fn read_actions(&self, client: &redis::Client) {
        let mut con = client.get_connection().unwrap();
        let actions: Option<Vec<PlayerAction>> = _redis::read_redis(&mut con, &self.uuid);
    }
}
