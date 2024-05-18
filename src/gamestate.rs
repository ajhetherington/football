use std::collections::HashMap;

use macroquad::{
    color::WHITE, prelude::QuadGl, text::draw_text, time::get_fps, window::clear_background,
};

use crate::{
    agent::{make_random_actions, AgentAction},
    render_pitch, Ball, Pitch, Team, VisiblePlayer,
};

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
    pub p_frame: i32,
    pub uuid: String,
    #[serde(skip)]
    pub players: Vec<VisiblePlayer>,
    pub actions: HashMap<String, AgentAction>,
    pub ball: Ball,
    pub team1: Team,
    pub team2: Team,
    pub pitch: Pitch,
    pub score: u8,
}

impl GameState {
    pub fn get_random_actions(&mut self) {
        self.actions = make_random_actions(&self.players)
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
