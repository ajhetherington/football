mod _redis;
mod ball;
mod gameobject;
mod getstate;
mod pitch;
mod player;
mod position;
mod team;
mod visibleplayer;
mod window;

use football::get_quad_gl;
use gameobject::GameObject;
use macroquad::prelude::*;
use std::{cell::RefCell, env};

use ball::*;
use pitch::*;
use player::Player;
use position::Position;
use serde;
use team::*;
use uuid::Uuid;
use visibleplayer::*;
use window::*;
use std::rc::Rc;

#[cfg(feature = "use_redis")]
use redis;

const PHYSICS_TICK_RATE: f32 = 1.0 / 30.0; // in seconds

fn setup_start_positions(players: &Vec<Player>, pitch: &Pitch, side: TeamSide) -> Vec<Position> {
    let player_x_position: f32;
    match side {
        TeamSide::Home => {
            player_x_position = pitch.width as f32 * 0.25;
        }
        TeamSide::Away => {
            player_x_position = pitch.width as f32 * 0.75;
        }
    }
    let padding: f32 = pitch.height as f32 / 10.0;
    let player_y_gap = (pitch.height - (2.0 * padding).floor() as i32) / (players.len() as i32 - 1);
    let mut positions: Vec<Position> = vec![Default::default(); players.len()];
    let mut multiplier: f32 = 0.0;
    // todo; positions objects are created later, so this can just be x & y
    for pos in positions.iter_mut() {
        *pos = Position {
            x: player_x_position + (pitch.x as f32),
            y: padding + (pitch.y as f32) + (player_y_gap as f32 * multiplier),
            prev_x: 0.0,
            prev_y: 0.0,
        };
        multiplier = multiplier + 1.0
    }
    positions
}

#[macroquad::main("Football")]
async fn main() {
    let game_id = Uuid::new_v4().to_string();
    let screen = ScreenSize::new();

    // set_window_size(screen.width as u32, screen.height as u32);
    let pitch = Pitch::new(&screen);

    let team1_players = Team::generate_players(5);
    let team_1_start_positions = setup_start_positions(&team1_players, &pitch, TeamSide::Home);
    let team1_visible_players = Team::make_visible(team1_players, team_1_start_positions);
    let team1_players: Vec<Rc<RefCell<VisiblePlayer>>> = team1_visible_players.iter().map(|val| Rc::new(RefCell::new(val.clone()))).collect();
    let mut team1 = Team::new(
        String::from("Team 1"),
        team1_players,
        TeamSide::Home,
    );
    let team2_players = Team::generate_players(5);
    let team_2_start_positions = setup_start_positions(&team2_players, &pitch, TeamSide::Away);
    let team2_visible_players = Team::make_visible(team2_players, team_2_start_positions);
    let team2_players: Vec<Rc<RefCell<VisiblePlayer>>> = team2_visible_players.iter().map(|val| Rc::new(RefCell::new(val.clone()))).collect();
    let mut team2 = Team::new(
        String::from("Team 2"),
        team2_players,
        TeamSide::Home,
    );

    const PHYSICS_TICK_RATE: f32 = 1.0 / 30.0; // in seconds
    let ball = Ball::new(130.0, 240.0);
    let mut time_accumulator: f32 = 0.0;
    team1.players[0].borrow_mut().to_movable();
    let score: u8 = 0;

    let mut all_players = team1.players.clone();
    all_players.extend(team2.players.clone());
    let mut state = GameState {
        p_frame: 0,
        uuid: game_id.clone(),
        players: all_players,
        pitch,
        ball,
        team1,
        team2,
        score,
    };

    #[cfg(feature = "use_redis")]
    let client = _redis::setup_redis(game_id);

    let qgl = get_quad_gl!();
    loop {
        time_accumulator += get_frame_time();

        while time_accumulator >= PHYSICS_TICK_RATE {
            #[cfg(feature = "use_redis")]
            state.log(&client);
            println!("applying physics");
            apply_physics_new(qgl, &mut state);
            time_accumulator -= PHYSICS_TICK_RATE;
        }
        let alpha: f32 = time_accumulator / PHYSICS_TICK_RATE;
        new_render(qgl, &mut state, alpha);
        next_frame().await
    }
}

fn apply_physics_new(qgl: &mut QuadGl, state: &mut GameState) {
    state.p_frame += 1;
    if is_key_down(KeyCode::Enter) {
        println!("kicking ball");
        state.ball.object.apply_force(8.0, -8.0, PHYSICS_TICK_RATE);
    }
    state.ball.object.apply_friction(PHYSICS_TICK_RATE);
    state
        .ball
        .object
        .update_position(&state.pitch, PHYSICS_TICK_RATE);
    for visibleplayer in state.team1.players.iter_mut() {
        if visibleplayer.borrow().is_movable() & is_mouse_button_down(macroquad::input::MouseButton::Left) {
            let (x, y) = macroquad::input::mouse_position();
            let x_dir = x - visibleplayer.borrow().object.pos.x;
            let y_dir = y - visibleplayer.borrow().object.pos.y;
            visibleplayer.borrow_mut().handle_kick_ball(&mut state.ball, x_dir, y_dir, PHYSICS_TICK_RATE);
        }
        visibleplayer.borrow_mut().new_handle_user_movement(qgl, PHYSICS_TICK_RATE);
        visibleplayer.borrow_mut().handle_physics(&state.pitch, PHYSICS_TICK_RATE);
    }
    for visibleplayer in state.team2.players.iter_mut() {
        if visibleplayer.borrow().is_movable() & is_mouse_button_down(macroquad::input::MouseButton::Left) {
            let (x, y) = macroquad::input::mouse_position();
            let x_dir = x - visibleplayer.borrow().object.pos.x;
            let y_dir = y - visibleplayer.borrow().object.pos.y;
            visibleplayer.borrow_mut().handle_kick_ball(&mut state.ball, x_dir, y_dir, PHYSICS_TICK_RATE);
        }
        visibleplayer.borrow_mut().new_handle_user_movement(qgl, PHYSICS_TICK_RATE);
        visibleplayer.borrow_mut().handle_physics(&state.pitch, PHYSICS_TICK_RATE);
    }
    // check collisions between players
    {
        let _radius = state.players[0].borrow().object.radius;


        
        // let mut player_objects: Vec<&mut GameObject> = vec![&mut team1_objects, &mut team2_objects].concat();
        let checks = gameobject::arange_checks(&mut state.players, _radius);
        // for (index_a, index_b) in checks {
        //     let value = player_objects[index_a].clone();
        //     let value_2 = player_objects[index_b].clone();
        //     gameobject::check_player_collisions(*value, *value_2, radius);
        // }
    }

    let goal = &state.pitch.check_goal(&state.ball);
    match goal {
        Some(g) => {
            println!("hi, there appears to be a goal for {:?}", g);
            match g {
                Goal::HOME => state.score = 1,
                Goal::AWAY => state.score = 2,
            }
        }
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
    pub players: Vec<Rc<RefCell<VisiblePlayer>>>,
    pub ball: Ball,
    pub team1: Team,
    pub team2: Team,
    pub pitch: Pitch,
    pub score: u8,
}

fn new_render(qgl: &mut QuadGl, state: &mut GameState, alpha: f32) {
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
    new_render_pitch(qgl, &state.pitch);
    for visibleplayer in state.team1.players.iter() {
        visibleplayer.borrow().new_render(qgl, alpha);
    }
    for visibleplayer in state.team2.players.iter() {
        visibleplayer.borrow().new_render(qgl, alpha);
    }
    draw_text(
        &format!("{}", get_fps()),
        100.0,
        12.0,
        10.0,
        macroquad::color::BLACK,
    );
    state.ball.new_render(qgl, alpha);
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
}

#[cfg(feature = "use_redis")]
impl LogRedis for GameState {
    fn log(&self, client: &redis::Client) {
        let mut con = client.get_connection().unwrap();
        let state_str = serde_json::to_string(&self).unwrap();
        _redis::write_redis(&mut con, &self.uuid, state_str.as_str());
    }
}

// fn render(
//     d: &mut RaylibDrawHandle,
//     ball: &mut Ball,
//     team1VisiblePlayers: &mut Vec<VisiblePlayer<'_>>,
//     team2VisiblePlayers: &mut Vec<VisiblePlayer<'_>>,
//     pitch: &Pitch,
//     alpha: f32,
//     score: u8,
// ) {
//     d.clear_background(Color::WHITE);
//     d.draw_text(
//         &format!(
//             "ball x: {:?}, y: {:?}",
//             ball.object.pos.x, ball.object.pos.y
//         ),
//         320,
//         12,
//         20,
//         Color::BLACK,
//     );
//     render_pitch(d, &pitch);
//     for visibleplayer in team1VisiblePlayers.iter() {
//         visibleplayer.draw(d, alpha);
//     }
//     for visibleplayer in team2VisiblePlayers.iter() {
//         visibleplayer.draw(d, alpha);
//     }
//     d.draw_text(&format!("{}", d.get_fps()), 100, 12, 10, Color::BLACK);
//     ball.display_ball(d, alpha);
//     d.draw_text(
//         &format!("Ball speed x: {}", ball.object.x_velocity),
//         200,
//         120,
//         10,
//         Color::BLACK,
//     );
//     d.draw_text(
//         &format!("Ball speed y: {}", ball.object.y_velocity),
//         200,
//         100,
//         10,
//         Color::BLACK,
//     );

//     if score > 0 {
//         d.draw_text(&format!("Score is {:?}", score), 10, 200, 5, Color::GOLD)
//     }
// }
