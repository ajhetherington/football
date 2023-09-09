use crate::player::Player;

enum PlayerActions {
    Shoot,
    Pass,
    Tackle,
    NoAction,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Position {
    pub x_position: f32,
    pub y_position: f32,
}

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
                    x_position: 0.0,
                    y_position: 0.0,
                },
            },
        }
    }
}
