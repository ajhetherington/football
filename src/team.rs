use football::MacroColour;
use macroquad::color::{BLUE, RED};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

use crate::player::Player;
use crate::position::Position;
use crate::visibleplayer::VisiblePlayer;
use crate::{pitch, Pitch};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum TeamSide {
    Home,
    Away,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    pub name: String,
    pub side: TeamSide,
    pub score: i32,
    #[serde(skip)]
    pub colour: MacroColour,
}

impl Team {
    pub fn new(name: String, team_side: TeamSide) -> Self {
        match team_side {
            TeamSide::Home => Team {
                name,
                side: team_side,
                score: 0,
                colour: football::MacroColour(RED),
            },
            TeamSide::Away => Team {
                name,
                side: team_side,
                score: 0,
                colour: football::MacroColour(BLUE),
            },
        }
    }


    pub fn generate_players(n_players: usize) -> Vec<Player> {
        //! Given usize n_players, will generate a vector of players
        //! Players are not given teams, that will be applied later
        let mut players: Vec<Player> = vec![];
        for i in 0..n_players {
            // todo: use faker to make fake names for players
            players.push(Player::new(format!("player-{}", i), i, None))
        }
        players
    }

    pub fn give_team(self, visible_player: &mut VisiblePlayer) {
        visible_player.player.team = Some(self.side);
    }

    fn setup_start_positions(self, players: &Vec<Player>, pitch: &Pitch) -> Vec<Position> {
        let player_x_position: f32;
        match self.side {
            TeamSide::Home => {
                player_x_position = pitch.width as f32 * 0.25;
            }
            TeamSide::Away => {
                player_x_position = pitch.width as f32 * 0.75;
            }
        }
        let padding: f32 = pitch.height as f32 / 10.0;
        let player_y_gap =
            (pitch.height - (2.0 * padding).floor() as i32) / (players.len() as i32 - 1);
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

    pub fn setup_team(&self, players: &Vec<Player>, pitch: &Pitch) -> Vec<VisiblePlayer> {
        let player_x_position: f32;
        match self.side {
            TeamSide::Home => {
                player_x_position = pitch.width as f32 * 0.25;
            }
            TeamSide::Away => {
                player_x_position = pitch.width as f32 * 0.75;
            }
        }
        let padding: f32 = pitch.height as f32 / 10.0;
        let player_y_gap =
            (pitch.height - (2.0 * padding).floor() as i32) / (players.len() as i32 - 1);
        // let mut positions: Vec<Position> = vec![Default::default(); players.len()];
        let mut multiplier: f32 = 0.0;
        players
            .iter()
            .map(|player| {
                let pos = Position {
                    x: player_x_position + (pitch.x as f32),
                    y: padding + (pitch.y as f32) + (player_y_gap as f32 * multiplier),
                    prev_x: 0.0,
                    prev_y: 0.0,
                };
                multiplier = multiplier + 1.0;
                return VisiblePlayer::new(player.clone(), pos.x, pos.y, self.colour.0);
            })
            .collect()
    }
}
