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
use macroquad::prelude::*;

use ball::*;
use pitch::*;
use player::Player;
use position::Position;
use team::*;
use visibleplayer::*;
use window::*;

const PHYSICS_TICK_RATE: f32 = 1.0 / 30.0; // in seconds

fn get_player_ref<'a>(players: &'a [Player; 5]) -> [&'a Player; 5] {
    [
        &players[0],
        &players[0],
        &players[0],
        &players[0],
        &players[0],
    ]
}

fn start_positions(team: &Team, pitch: &Pitch) -> [Position; 5] {
    let player_x_position: f32;
    match team.side {
        TeamSide::Home => {
            player_x_position = pitch.width as f32 * 0.25;
        }
        TeamSide::Away => {
            player_x_position = pitch.width as f32 * 0.75;
        }
    }
    let padding: f32 = pitch.height as f32 / 10.0;
    let player_y_gap =
        (pitch.height - (2.0 * padding).floor() as i32) / (team.players.len() as i32 - 1);
    let mut positions: [Position; 5] = Default::default();
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
    let screen = ScreenSize::new();

    // set_window_size(screen.width as u32, screen.height as u32);
    let pitch = Pitch::new(&screen);

    let team1Players = Team::generate_players();
    let team1 = Team::new(
        String::from("Team 1"),
        get_player_ref(&team1Players),
        TeamSide::Home,
    );
    let start_positions_team_1 = start_positions(&team1, &pitch);
    let mut team1VisiblePlayers: Vec<VisiblePlayer> = Vec::new();
    for (position, player) in start_positions_team_1.iter().zip(team1Players.iter()) {
        team1VisiblePlayers.push(VisiblePlayer::new(player, position.x, position.y, RED))
    }

    let team2Players = Team::generate_players();
    let team2 = Team::new(
        String::from("Team 2"),
        get_player_ref(&team2Players),
        TeamSide::Away,
    );
    let start_positions_team_2 = start_positions(&team2, &pitch);
    let mut team2VisiblePlayers: Vec<VisiblePlayer> = Vec::new();
    for (position, player) in start_positions_team_2.iter().zip(team1Players.iter()) {
        team2VisiblePlayers.push(VisiblePlayer::new(player, position.x, position.y, BLACK))
    }

    const PHYSICS_TICK_RATE: f32 = 1.0 / 30.0; // in seconds
    let mut ball = Ball::new(130.0, 240.0);
    let mut time_accumulator: f32 = 0.0;
    team1VisiblePlayers[0].to_movable();
    let score: u8 = 0;

    let mut state = GameState {
        pitch: &pitch,
        ball: &mut ball,
        team1_visible_players: &mut team1VisiblePlayers,
        team2_visible_players: &mut team2VisiblePlayers,
        score,
    };

    let qgl = get_quad_gl!();
    loop {
        time_accumulator += get_frame_time();
        while time_accumulator >= PHYSICS_TICK_RATE {
            apply_physics_new(qgl, &mut state);
            time_accumulator -= PHYSICS_TICK_RATE;
        }
        let alpha: f32 = time_accumulator / PHYSICS_TICK_RATE;
        new_render(qgl, &mut state, alpha);
        next_frame().await
    }
}

// fn log_game_state(con: &mut redis::Connection, ball: &Ball, team1VisiblePlayers: & Vec<VisiblePlayer<'_>>, team2VisiblePlayers: & Vec<VisiblePlayer<'_>>, pitch: &Pitch, score: u8) {
//     ball.log_in_redis(con, "ball");
//     for player in team1VisiblePlayers.into_iter() {
//         player.log_in_redis(con, player.player.name.as_str());
//     }
//     for player in team2VisiblePlayers.into_iter() {
//         player.log_in_redis(con, player.player.name.as_str());
//     }
//     // let idk = con.publish("channel1", ball.object.pos.x);
//     // match idk {
//     //     Ok(value) => value,
//     //     _ => {}
//     // }
// }

// pub trait LogInRedis {
//     fn log_in_redis(&self, con: &mut redis::Connection, channel: &str)
//     where
//         Self: serde::Serialize {
//         let json_string = serde_json::to_string(&self).expect("Failed to deserialize ball");
//         let succ = con.publish(channel, json_string);
//         match succ {
//             Ok(value) => value,
//             _ => panic!("Failed to write to redis for ball")
//         };

//     }
// }
// impl LogInRedis for Ball {}
// impl LogInRedis for Player {}
// impl LogInRedis for VisiblePlayer<'_> {}

// ### by using the default implementation above (using the where Self: serde::Serialize)
// ### I was able to remove the redundent boilerplate below

// impl LogInRedis for Ball {
//     fn log_in_redis(&self, con: &mut redis::Connection) {
//         let json_string = serde_json::to_string(&self).expect("Failed to deserialize ball");
//         let succ = con.publish("channel1", json_string);
//         match succ {
//             Ok(value) => value,
//             _ => panic!("Failed to write to redis for ball")
//         };
//     }
// }

// impl LogInRedis for Player {
//     fn log_in_redis(&self, con: &mut redis::Connection) {
//         let json_string = serde_json::to_string(&self).expect("Failed to deserialize ball");
//         let succ = con.publish("channel1", json_string);
//         match succ {
//             Ok(value) => value,
//             _ => panic!("Failed to write to redis for ball")
//         };
//     }
// }

fn apply_physics_new(qgl: &mut QuadGl, state: &mut GameState) {
    if is_key_down(KeyCode::Enter) {
        println!("kicking ball");
        state.ball.object.apply_force(8.0, -8.0, PHYSICS_TICK_RATE);
    }
    state.ball.object.apply_friction(PHYSICS_TICK_RATE);
    state
        .ball
        .object
        .update_position(&state.pitch, PHYSICS_TICK_RATE);
    for visibleplayer in state.team1_visible_players.iter_mut() {
        if visibleplayer.is_movable() & is_mouse_button_down(macroquad::input::MouseButton::Left) {
            let (x, y) = macroquad::input::mouse_position();
            let x_dir = x - visibleplayer.object.pos.x;
            let y_dir = y - visibleplayer.object.pos.y;
            visibleplayer.handle_kick_ball(&mut state.ball, x_dir, y_dir, PHYSICS_TICK_RATE);
        }
        visibleplayer.new_handle_user_movement(qgl, PHYSICS_TICK_RATE);
        visibleplayer.handle_physics(&state.pitch, PHYSICS_TICK_RATE);
    }
    for visibleplayer in state.team2_visible_players.iter_mut() {
        if visibleplayer.is_movable() & is_mouse_button_down(macroquad::input::MouseButton::Left) {
            let (x, y) = macroquad::input::mouse_position();
            let x_dir = x - visibleplayer.object.pos.x;
            let y_dir = y - visibleplayer.object.pos.y;
            visibleplayer.handle_kick_ball(&mut state.ball, x_dir, y_dir, PHYSICS_TICK_RATE);
        }
        visibleplayer.new_handle_user_movement(qgl, PHYSICS_TICK_RATE);
        visibleplayer.handle_physics(&state.pitch, PHYSICS_TICK_RATE);
    }
}

// fn apply_physics(
//     rl: &mut RaylibHandle,
//     ball: &mut Ball,
//     team1VisiblePlayers: &mut Vec<VisiblePlayer<'_>>,
//     team2VisiblePlayers: &mut Vec<VisiblePlayer<'_>>,
//     pitch: &Pitch,
//     score: &mut u8,
// ) {
//     if rl.is_key_down(KeyboardKey::KEY_ENTER) {
//         println!("kicking ball");
//         // todo: going thing.object.apply_force is cumbersome
//         // can instead use traits to add apply_force method to both ball & visibleplayer
//         ball.object.apply_force(8.0, -8.0, PHYSICS_TICK_RATE);
//     }
//     ball.object.apply_friction(PHYSICS_TICK_RATE);
//     ball.object.update_position(&pitch, PHYSICS_TICK_RATE);
//     for visibleplayer in team1VisiblePlayers.iter_mut() {
//         if visibleplayer.is_movable() & rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
//             let x_dir = rl.get_mouse_x().as_f32() - visibleplayer.object.pos.x;
//             let y_dir = rl.get_mouse_y().as_f32() - visibleplayer.object.pos.y;
//             visibleplayer.handle_kick_ball(ball, x_dir, y_dir, PHYSICS_TICK_RATE);
//         }
//         visibleplayer.handle_user_movement(rl, PHYSICS_TICK_RATE);
//         visibleplayer.handle_physics(&pitch, PHYSICS_TICK_RATE);
//     }
//     for visibleplayer in team2VisiblePlayers.iter_mut() {
//         if visibleplayer.is_movable() & rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
//             let x_dir = rl.get_mouse_x().as_f32() - visibleplayer.object.pos.x;
//             let y_dir = rl.get_mouse_y().as_f32() - visibleplayer.object.pos.y;
//             visibleplayer.handle_kick_ball(ball, x_dir, y_dir, PHYSICS_TICK_RATE);
//         }
//         visibleplayer.handle_user_movement(rl, PHYSICS_TICK_RATE);
//         visibleplayer.handle_physics(&pitch, PHYSICS_TICK_RATE);
//     }
//     let goal = pitch.check_goal(&ball);
//     match goal {
//         Some(g) => {
//             println!("hi, there appears to be a goal for {:?}", g);
//             match g {
//                 Goal::HOME => *score = 1,
//                 Goal::AWAY => *score = 2,
//             }
//         }
//         _ => {}
//     }
//     // should always log state at the end of the game
// }

pub struct GameState<'a> {
    pub ball: &'a mut Ball,
    pub team1_visible_players: &'a mut Vec<VisiblePlayer<'a>>,
    pub team2_visible_players: &'a mut Vec<VisiblePlayer<'a>>,
    pub pitch: &'a Pitch,
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
    new_render_pitch(qgl, state.pitch);
    for visibleplayer in state.team1_visible_players.iter() {
        visibleplayer.new_render(qgl, alpha);
    }
    for visibleplayer in state.team2_visible_players.iter() {
        visibleplayer.new_render(qgl, alpha);
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
