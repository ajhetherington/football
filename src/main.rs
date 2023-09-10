mod ball;
mod gameobject;
mod pitch;
mod player;
mod position;
mod team;
mod visibleplayer;
mod window;
use ball::*;
use pitch::*;
use player::Player;
use position::Position;
use raylib::{misc::AsF32, prelude::*};
use team::*;
use visibleplayer::*;
use window::*;

fn get_player_ref<'a>(players: &'a [Player; 5]) -> [&'a Player; 5] {
    [
        &players[0],
        &players[0],
        &players[0],
        &players[0],
        &players[0],
    ]
}

fn main() {
    for _ in 1..20 {
        let me = Player::new(String::from("alex"), 10).unwrap();
        println!("This is the player {:?}", me);
    }

    let untied_players = Team::generate_players();

    let dyslexia_untied = Team::new(
        String::from("Dyslexia Untied"),
        get_player_ref(&untied_players),
        TeamSide::Home,
    );

    println!("This is the team {:?}", dyslexia_untied);

    render_something()
}

fn start_positions(team: Team, pitch: &Pitch) -> [Position; 5] {
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

pub fn render_something() {
    let screen = ScreenSize::new();
    let (mut rl, thread) = raylib::init()
        .size(screen.width as i32, screen.height as i32)
        .title("Not just Football Manager")
        .msaa_4x()
        .build();

    // rl.set_target_fps(120);

    let mut ball_position: Vec<f32> = vec![screen.width as f32 / 2.0, screen.height as f32 / 2.0];
    let pitch = Pitch::new(&screen);

    let team1Players = Team::generate_players();
    let team1 = Team::new(
        String::from("Team 1"),
        get_player_ref(&team1Players),
        TeamSide::Home,
    );
    let start_positions_team_1 = start_positions(team1, &pitch);
    println!("{:?}", start_positions_team_1);
    let mut team1VisiblePlayers: Vec<VisiblePlayer> = Vec::new();
    for (position, player) in start_positions_team_1.iter().zip(team1Players.iter()) {
        team1VisiblePlayers.push(VisiblePlayer::new(
            player,
            position.x,
            position.y,
            Color::RED,
        ))
    }

    let team2Players = Team::generate_players();
    let team2 = Team::new(
        String::from("Team 2"),
        get_player_ref(&team2Players),
        TeamSide::Away,
    );
    let start_positions_team_2 = start_positions(team2, &pitch);
    let mut team2VisiblePlayers: Vec<VisiblePlayer> = Vec::new();
    for (position, player) in start_positions_team_2.iter().zip(team1Players.iter()) {
        team2VisiblePlayers.push(VisiblePlayer::new(
            player,
            position.x,
            position.y,
            Color::BLUE,
        ))
    }

    const MOVE_SPEED: f32 = 0.5;
    const PLAYER_RADIUS: f32 = 10.0;
    let ball_radius: f32 = PLAYER_RADIUS;

    let mut ball = Ball::new(130.0, 240.0);
    const PHYSICS_TICK_RATE: f32 = 1.0 / 30.0; // in seconds
    let mut time_accumulator: f32 = 0.0;
    team1VisiblePlayers[0].to_movable();

    // the renderer produces time and the simulation consumes it in discrete dt sized steps
    while !rl.window_should_close() {
        time_accumulator += rl.get_frame_time();

        while time_accumulator >= PHYSICS_TICK_RATE {
            if rl.is_key_down(KeyboardKey::KEY_ENTER) {
                println!("kicking ball");
                // todo: going thing.object.apply_force is cumbersome
                // can instead use traits to add apply_force method to both ball & visibleplayer
                ball.object.apply_force(8.0, -8.0, PHYSICS_TICK_RATE);
            }
            ball.object.apply_friction(PHYSICS_TICK_RATE);
            ball.object.update_position(&pitch, PHYSICS_TICK_RATE);

            for visibleplayer in team1VisiblePlayers.iter_mut() {
                visibleplayer.handle_user_movement(&mut rl, PHYSICS_TICK_RATE);
                visibleplayer.handle_physics(&pitch, PHYSICS_TICK_RATE);
                if visibleplayer.is_movable() & rl.is_key_down(KeyboardKey::KEY_X) {
                    let x_dir = rl.get_mouse_x().as_f32() - visibleplayer.object.pos.x;
                    let y_dir = rl.get_mouse_y().as_f32() - visibleplayer.object.pos.y;
                    visibleplayer.handle_kick_ball(
                        &pitch,
                        &mut ball,
                        x_dir,
                        y_dir,
                        PHYSICS_TICK_RATE,
                    );
                }
            }
            for visibleplayer in team2VisiblePlayers.iter_mut() {
                visibleplayer.handle_user_movement(&mut rl, PHYSICS_TICK_RATE);
                visibleplayer.handle_physics(&pitch, PHYSICS_TICK_RATE);
                if visibleplayer.is_movable() & rl.is_key_down(KeyboardKey::KEY_X) {
                    let x_dir = rl.get_mouse_x().as_f32() - visibleplayer.object.pos.x;
                    let y_dir = rl.get_mouse_y().as_f32() - visibleplayer.object.pos.y;
                    visibleplayer.handle_kick_ball(
                        &pitch,
                        &mut ball,
                        x_dir,
                        y_dir,
                        PHYSICS_TICK_RATE,
                    );
                }
            }

            move_circle_arrow(&mut rl, &mut ball_position, MOVE_SPEED);

            time_accumulator -= PHYSICS_TICK_RATE;
        }

        let alpha = time_accumulator / PHYSICS_TICK_RATE;

        // then apply part of velocity to position
        // ball.update_position(&pitch, alpha);

        if ball_position[0] < ball_radius {
            ball_position[0] = ball_radius;
        }
        if ball_position[0] > (screen.width as f32) - ball_radius {
            ball_position[0] = (screen.width as f32) - ball_radius;
        }
        if ball_position[1] < ball_radius {
            ball_position[1] = ball_radius;
        }
        if ball_position[1] > (screen.height as f32) - ball_radius {
            ball_position[1] = (screen.height as f32) - ball_radius;
        }

        // display / render
        // only use interpolate when rendering, don't update actual position states
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world", 12, 12, 20, Color::BLACK);
        d.draw_text(
            &format!("x: {:?}, y: {:?}", ball_position[0], ball_position[1]),
            320,
            12,
            20,
            Color::BLACK,
        );
        render_pitch(&mut d, &pitch);
        for visibleplayer in team1VisiblePlayers.iter() {
            visibleplayer.draw(&mut d, alpha);
        }
        for visibleplayer in team2VisiblePlayers.iter() {
            visibleplayer.draw(&mut d, alpha);
        }
        d.draw_text(&format!("{}", d.get_fps()), 100, 12, 10, Color::BLACK);
        d.draw_circle(
            ball_position[0].floor() as i32,
            ball_position[1].floor() as i32,
            ball_radius,
            Color::ORANGE,
        );
        ball.display_ball(&mut d, alpha);
        d.draw_text(
            &format!("Ball speed x: {}", ball.object.x_velocity),
            200,
            120,
            10,
            Color::BLACK,
        );
        d.draw_text(
            &format!("Ball speed y: {}", ball.object.y_velocity),
            200,
            100,
            10,
            Color::BLACK,
        );
    }
}

fn move_circle_arrow(rl: &mut RaylibHandle, ball_position: &mut Vec<f32>, MOVE_SPEED: f32) {
    if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
        ball_position[0] = ball_position[0] + MOVE_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_LEFT) {
        ball_position[0] = ball_position[0] - MOVE_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_UP) {
        ball_position[1] = ball_position[1] - MOVE_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_DOWN) {
        ball_position[1] = ball_position[1] + MOVE_SPEED;
    }
}
