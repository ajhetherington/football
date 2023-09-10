use crate::gameobject::GameObject;
use crate::position::Position;
use raylib::core::drawing::RaylibDrawHandle;
use raylib::prelude::{Color, RaylibDraw};

#[derive(Debug)]
pub struct Ball {
    pub object: GameObject,
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Self {
        return Ball {
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
                mass: 0.30,
                friction: 0.99,
            },
        };
    }

    pub fn display_ball(&mut self, d: &mut RaylibDrawHandle, alpha: f32) {
        d.draw_circle(
            self.object.pos.interpolate_x(alpha),
            self.object.pos.interpolate_y(alpha),
            self.object.radius,
            Color::GREEN,
        )
    }
}
