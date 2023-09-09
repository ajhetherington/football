mod player;
mod visibleplayer;
mod team;
use player::Player;
use raylib::prelude::*;
use visibleplayer::*;
use team::*;

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

struct ScreenSize {
    width: usize,
    height: usize,
}
impl ScreenSize {
    pub fn new() -> Self {
        ScreenSize {
            width: (640),
            height: (480),
        }
    }
}

fn start_positions(team: Team, screen: &ScreenSize) -> [Position; 5] {
    let player_x_position: f32;
    match team.side {
        TeamSide::Home => {
            player_x_position = screen.width as f32 * 0.25;
        }
        TeamSide::Away => {
            player_x_position = screen.width as f32 * 0.75;
        }
    }
    let padding: f32 = screen.height as f32 / 10.0;
    let player_y_gap = (screen.height - (2.0 * padding).floor() as usize) / team.players.len();
    let mut positions: [Position; 5] = Default::default();
    let mut multiplier: f32 = 0.0;
    for pos in positions.iter_mut() {
        *pos = Position {
            x_position: player_x_position,
            y_position: padding + (player_y_gap as f32 * multiplier),
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
        .build();

    let mut ball_position: Vec<f32> = vec![screen.width as f32 / 2.0, screen.height as f32 / 2.0];

    let team1Players = Team::generate_players();
    let team1 = Team::new(
        String::from("Team 1"),
        get_player_ref(&team1Players),
        TeamSide::Home,
    );
    let start_positions_team_1 = start_positions(team1, &screen);
    println!("{:?}", start_positions_team_1);
    let mut team1VisiblePlayers: Vec<VisiblePlayer> = Vec::new();
    for (position, player) in start_positions_team_1.iter().zip(team1Players.iter()) {
        team1VisiblePlayers.push(VisiblePlayer {
            position: *position,
            player,
        })
    }

    let team2Players = Team::generate_players();
    let team2 = Team::new(
        String::from("Team 2"),
        get_player_ref(&team2Players),
        TeamSide::Away,
    );
    let start_positions_team_2 = start_positions(team2, &screen);
    let mut team2VisiblePlayers: Vec<VisiblePlayer> = Vec::new();
    for (position, player) in start_positions_team_2.iter().zip(team1Players.iter()) {
        team2VisiblePlayers.push(VisiblePlayer {
            position: *position,
            player,
        })
    }

    const MOVE_SPEED: f32 = 0.5;
    const PLAYER_RADIUS: f32 = 10.0;
    let ball_radius: f32 = PLAYER_RADIUS;

    while !rl.window_should_close() {
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

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world", 12, 12, 20, Color::BLACK);
        for visibleplayer in team1VisiblePlayers.iter() {
            d.draw_circle(
                visibleplayer.position.x_position.floor() as i32,
                visibleplayer.position.y_position.floor() as i32,
                PLAYER_RADIUS,
                Color::RED,
            )
        }
        for visibleplayer in team2VisiblePlayers.iter() {
            d.draw_circle(
                visibleplayer.position.x_position.floor() as i32,
                visibleplayer.position.y_position.floor() as i32,
                PLAYER_RADIUS,
                Color::BLUE,
            )
        }
        d.draw_circle(
            ball_position[0].floor() as i32,
            ball_position[1].floor() as i32,
            ball_radius,
            Color::ORANGE,
        );
    }
}
