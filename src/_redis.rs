use std::{env, num::NonZeroUsize};
#[cfg(feature="use_redis")]
use redis::{self, Commands};


#[cfg(feature="use_redis")]
pub fn setup_redis(uuid: String) -> redis::Client {
    let redis_str = env::var("REDIS_STR").unwrap_or("redis://127.0.0.1:6379".to_owned());
    let client = redis::Client::open(redis_str).unwrap();

    client
}

#[cfg(feature="use_redis")]
pub fn write_redis(con: &mut redis::Connection, uuid: &String, message: &str) -> bool {
    let state_str = format!("{_uuid}:state", _uuid=uuid.to_string());
    match con.rpush::<String, String, String>(state_str, message.to_owned()) {
        Ok(_val) =>  true,
        Err(_e) => false
    }
}

#[cfg(feature="use_redis")]
pub fn read_redis(con: &mut redis::Connection, uuid: &String) -> String {
    let action_str = format!("{_uuid}:action", _uuid=uuid.to_string());
    match con.lpop(action_str, NonZeroUsize::new(0)) {
        Ok(val) => val,
        Err(e) => {
            return "".to_owned()
        }
    }

}

