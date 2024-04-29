use serde::{Deserialize, Serialize};
#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub prev_x: f32,
    pub prev_y: f32
}

impl Position {
    pub fn interpolate_x(&self, alpha: f32) -> f32 {
        ((self.x * alpha) + (self.prev_x * (1.0 - alpha))).round()
    }
    pub fn interpolate_y(&self, alpha: f32) -> f32 {
        ((self.y * alpha) + (self.prev_y * (1.0 - alpha))).round()
    }
}