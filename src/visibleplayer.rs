use crate::player::Player;
use crate::position::Position;

enum PlayerActions {
    Shoot,
    Pass,
    Tackle,
    NoAction,
}

#[derive(Clone, Copy, Debug)]

pub struct VisiblePlayer<'a> {
    pub position: Position,
    pub player: &'a Player,
}

impl<'a> VisiblePlayer<'a> {
    pub fn new(player: &'a Player, position: Option<Position>) -> Self {
        match position {
            Some(position) => VisiblePlayer { player, position },
            None => VisiblePlayer {
                player,
                position: Position {
                    x: 0.0,
                    y: 0.0,
                    prev_x: 0.0,
                    prev_y: 0.0
                },
            },
        }
    }
}
