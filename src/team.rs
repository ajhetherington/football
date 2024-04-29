use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::cell::RefCell;

use crate::player::Player;
use crate::position::Position;
use crate::visibleplayer::VisiblePlayer;

#[derive(Serialize, Deserialize, Debug)]
pub enum TeamSide {
    Home,
    Away,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    pub name: String,
    // todo: implement serialization for this
    #[serde(skip)] 
    pub players: Vec<Rc<RefCell<VisiblePlayer>>>, // Array of references to 5 player objects
    pub side: TeamSide,
}

impl Team {
    pub fn new(
        name: String,
        players: Vec<Rc<RefCell<VisiblePlayer>>>,
        team_side: TeamSide,
    ) -> Self {

        Team {
            name,
            players: players,
            side: team_side,
        }
    }
    
    pub fn make_visible(players: Vec<Player>, positions: Vec<Position>) -> Vec<VisiblePlayer> {
        let mut visible_players: Vec<VisiblePlayer> = Vec::new();
        for (position, player) in positions.iter().zip(players.iter()) {
            visible_players.push(VisiblePlayer::new(
                player.clone(), // bc i am lazy
                position.x,
                position.y,
                macroquad::color::RED,
            ))
        }
        visible_players

    }

    pub fn generate_players(n_players: usize) -> Vec<Player> {
        // VisiblePlayer::new(player, x, y, color)
        let mut players: Vec<Player> = vec![];
        for i in 0..n_players {
            // todo: use faker to make fake names for players
            players.push(Player::new(format!("player-{}", i), i))
        }
        players
    }
}
