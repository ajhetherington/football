use crate::player::Player;


pub trait RedisState {
    fn get_state(&self) {

    }
}

impl RedisState for &Player {
    fn get_state(&self) {
        println!("{:?}", self.name)
    }
}