use crate::{gameobject::GameObject, VisiblePlayer};
use macroquad::{prelude::*, rand};
use macroquad::shapes::draw_circle;
use serde::{Deserialize, Serialize};

const KICK_RADIUS: f32 = 30.0;

#[derive(Debug, Deserialize, Serialize)]
pub struct Ball {
    pub object: GameObject,
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Self {
        return Ball {
            object: GameObject::new(x, y, 6.0, 0.3, 0.99),
        };
    }

    pub fn render(&mut self, _qgl: &mut QuadGl, alpha: f32) {
        draw_circle(
            self.object.pos.interpolate_x(alpha),
            self.object.pos.interpolate_y(alpha),
            self.object.radius,
            macroquad::color::GREEN,
        )
    }
    pub fn is_kickable_by(&self, player: &VisiblePlayer) -> bool {
        (player.object.pos.x - self.object.pos.x).abs() < KICK_RADIUS
            && (player.object.pos.y - self.object.pos.y).abs() < KICK_RADIUS
    }

    pub fn game_start_kick(&mut self, dt: f32) {
        let force_x: f32 = rand::gen_range(-200.0, 200.0);
        let force_y: f32 = rand::gen_range(-200.0, 200.0);
        println!("force x: {force_x}");
        println!("force y: {force_y}");
        self.object.apply_force(force_x, force_y, dt);
    }
}
