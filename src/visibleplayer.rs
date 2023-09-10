use crate::gameobject::GameObject;
use crate::player::Player;
use crate::position::Position;
use raylib::core::drawing::RaylibDrawHandle;
use raylib::prelude::{Color, RaylibDraw};

enum PlayerActions {
    Shoot,
    Pass,
    Tackle,
    NoAction,
}

#[derive(Clone, Copy, Debug)]

pub struct VisiblePlayer<'a> {
    pub player: &'a Player,
    pub object: GameObject,
    movable: bool,
    color: Color
}

impl<'a> VisiblePlayer<'a> {
    pub fn new(player: &'a Player, x: f32, y: f32, color: Color) -> Self {
        return VisiblePlayer {
            player,
            object: GameObject {
                pos: Position {
                    x,
                    y,
                    prev_x: x,
                    prev_y: y,
                },
                x_velocity: 0.0,
                y_velocity: 0.0,
                radius: 8.0,
                mass: 2.0,
                friction: 0.8,
            },
            color,
            movable: false,
        };
    }
    pub fn to_movable(&mut self) {
        self.movable = true;
    }

    pub fn draw(& self, d: &mut RaylibDrawHandle, alpha: f32) {
        d.draw_circle(
            self.object.pos.interpolate_x(alpha),
            self.object.pos.interpolate_y(alpha),
            self.object.radius,
            self.color,
        )
    }
}
