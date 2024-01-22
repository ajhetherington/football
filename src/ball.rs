use crate::gameobject::GameObject;
use crate::position::Position;
use macroquad::prelude::*;
use macroquad::shapes::draw_circle;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Ball {
    pub object: GameObject,
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Self {
        return Ball {
            object: GameObject::new(x, y, 8.0, 0.3, 0.99),
        };
    }

    pub fn new_render(&mut self, _qgl: &mut QuadGl, alpha: f32) {
        draw_circle(
            self.object.pos.interpolate_x(alpha),
            self.object.pos.interpolate_y(alpha),
            self.object.radius,
            macroquad::color::GREEN,
        )
    }
}
