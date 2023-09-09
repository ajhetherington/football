use raylib::core::drawing::RaylibDrawHandle;
use raylib::prelude::{Color, RaylibDraw};
use crate::pitch::Pitch;

pub struct Ball {
    x: i32,
    y: i32,
    x_velocity: f32,
    y_velocity: f32,
    radius: i32,
    mass: f32,
    friction: f32,
}

impl Ball {
    pub fn new(x: i32, y: i32) -> Self {
        return Ball {
            x,
            y,
            x_velocity: 0.0,
            y_velocity: 0.0,
            radius: 8,
            mass: 0.2,
            friction: 0.99,
        };
    }

    pub fn kick(&mut self, force_x: f32, force_y: f32) {
        let accel_x = force_x / self.mass;
        let accel_y = force_y / self.mass;
        self.x_velocity += accel_x;
        self.y_velocity += accel_y;
    }

    fn apply_friction(&mut self) {
        self.x_velocity *= self.friction;
        self.y_velocity *= self.friction;
    }
    pub fn update_position(&mut self, pitch: &Pitch) {
        self.apply_friction();
        let mut updated_x_position = self.x + self.x_velocity.round() as i32;
        let mut updated_y_position = self.y + self.y_velocity.round() as i32;

        if updated_x_position <= pitch.x + self.radius {
            updated_x_position = pitch.x + self.radius;
            self.x_velocity = -0.9 * self.x_velocity;
        } else if updated_x_position >= (pitch.x + pitch.width) - self.radius {
            updated_x_position = (pitch.x + pitch.width) - self.radius;
            self.x_velocity = -0.9 * self.x_velocity;
        }

        if updated_y_position <= pitch.y + self.radius {
            updated_y_position = pitch.y + self.radius;
            self.y_velocity = -0.9 * self.y_velocity;
        } else if updated_y_position >= (pitch.y + pitch.height) - self.radius {
            updated_y_position = (pitch.y + pitch.height) - self.radius;
            self.y_velocity = -0.9 * self.y_velocity;
        }

        self.x = updated_x_position;
        self.y = updated_y_position;

    }
    pub fn display_ball(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_circle(self.x, self.y, self.radius as f32, Color::GREEN)
    }
}
