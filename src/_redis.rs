#[cfg(feature = "use_redis")]
use redis::{self, Commands};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{env, num::NonZeroUsize};

use crate::player::Player;

#[derive(Serialize, Deserialize)]
pub struct PlayerAction {
    player: i32,
    distance: f32,
    angle_rads: f32,
}

#[cfg(feature = "use_redis")]
pub fn setup_redis(uuid: String) -> redis::Client {
    let redis_str = env::var("REDIS_STR").unwrap_or("redis://127.0.0.1:6379".to_owned());
    let client = redis::Client::open(redis_str).unwrap();

    client
}

#[cfg(feature = "use_redis")]
pub fn write_redis(con: &mut redis::Connection, uuid: &String, message: &str) -> bool {
    let state_str = format!("{_uuid}:state", _uuid = uuid.to_string());
    match con.rpush::<String, String, String>(state_str, message.to_owned()) {
        Ok(_val) => true,
        Err(_e) => false,
    }
}

#[cfg(feature = "use_redis")]
pub fn read_redis(con: &mut redis::Connection, uuid: &String) -> Option<Vec<PlayerAction>> {
    let action_key = format!("{_uuid}:action", _uuid = uuid.to_string());
    let action_value: Option<String> = match con.lpop(action_key, NonZeroUsize::new(0)) {
        Ok(val) => val,
        Err(_e) => None,
    };
    match action_value {
        Some(val) => Some(serde_json::from_str(val.as_str()).unwrap()),
        None => None,
    }
}
