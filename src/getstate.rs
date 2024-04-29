use crate::visibleplayer::VisiblePlayer;


pub trait RedisState {
    fn get_state(&self) -> String;
}

impl<'a> RedisState for VisiblePlayer {
    fn get_state(&self) -> String {
        self.object.pos.x.to_string()
    }
}