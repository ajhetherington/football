use macroquad::rand;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash};

use crate::VisiblePlayer;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AgentAction {
    /// A full schema of the action directive that an agent
    /// should respond with to the environment
    pub x: f32,
    pub y: f32,
    pub kick: bool,
    // Here have array of actions, so player can move 8 directions
    pub movement: Vec<MovementAction>,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Debug)]
pub enum MovementAction {
    /// Directions that a player can move
    Up,
    Down,
    Left,
    Right,
}

pub fn make_random_actions(players: &Vec<VisiblePlayer>) -> HashMap<String, AgentAction> {
    let mut ret_map = HashMap::new();
    for player in players.iter() {
        ret_map.insert(player.player.uuid.clone(), random_action());
    }
    return ret_map;
}

pub fn random_action() -> AgentAction {
    //! Generates a random action
    let x: f32 = rand::RandomRange::gen_range(0.0, 10.0) - 5.0;
    let y: f32 = rand::RandomRange::gen_range(0.0, 10.0) - 5.0;
    let kick_float: f32 = rand::RandomRange::gen_range(0.0, 1.0);
    let kick: bool = kick_float.round() != 0.0;

    let movement = match rand::gen_range(0, 3) {
    0 => vec![MovementAction::Up],
        1 => vec![MovementAction::Down],
        2 => vec![MovementAction::Left],
        3 => vec![MovementAction::Right],
        _ => vec![],
    };
    return AgentAction {
        x,
        y,
        kick,
        movement,
    };
}
