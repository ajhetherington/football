use raylib::core::drawing::RaylibDrawHandle;
use raylib::prelude::{Color, RaylibDraw};
use crate::pitch::Pitch;
use crate::position::Position;


#[derive(Debug)]
pub struct Ball {
    pos: Position,
    x_velocity: f32,
    y_velocity: f32,
    radius: f32,
    mass: f32,
    friction: f32,
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Self {
        return Ball {
            pos: Position {x, y, prev_x: 0.0,prev_y: 0.0},
            x_velocity: 0.0,
            y_velocity: 0.0,
            radius: 8.0,
            mass: 2.0,
            friction: 0.95,
        };
    }
    pub fn speed_x(&mut self) -> f32 {
        self.x_velocity.abs()
    }
    pub fn speed_y(&mut self) -> f32 {
        self.y_velocity.abs()
    }

    pub fn speed(&mut self) -> f32 {
        ((2.0 as f32).powf(self.x_velocity).sqrt() +  (2.0 as f32).powf(self.y_velocity).sqrt()) / 2.0
    }

    pub fn kick(&mut self, force_x: f32, force_y: f32, dt: f32) {
        let accel_x = force_x / self.mass;
        let accel_y = force_y / self.mass;
        self.x_velocity += dt * accel_x;
        self.y_velocity += dt * accel_y;
        println!("{:?}", self);
    }

    pub fn apply_friction(&mut self) {
        self.x_velocity *= self.friction;
        self.y_velocity *= self.friction;
        self.x_velocity = (self.x_velocity * 10000.0).floor() / 10000.0;
        self.y_velocity = (self.y_velocity * 10000.0).floor() / 10000.0;
    }

    pub fn update_position(&mut self, pitch: &Pitch, dt: f32) {
        let mut updated_x_position = self.pos.x + (self.x_velocity * dt);
        let mut updated_y_position = self.pos.y + (self.y_velocity * dt);

        if updated_x_position <= (pitch.x as f32) + self.radius {
            updated_x_position = (pitch.x as f32) + self.radius;
            self.x_velocity = -0.9 * self.x_velocity;
        } else if updated_x_position >= ((pitch.x + pitch.width) as f32) - self.radius {
            updated_x_position = ((pitch.x + pitch.width) as f32) - self.radius;
            self.x_velocity = -0.9 * self.x_velocity;
        }

        if updated_y_position <= (pitch.y as f32) + self.radius {
            updated_y_position = (pitch.y as f32) + self.radius;
            self.y_velocity = -0.9 * self.y_velocity;
        } else if updated_y_position >= ((pitch.y + pitch.height) as f32) - self.radius {
            updated_y_position = ((pitch.y + pitch.height) as f32) - self.radius;
            self.y_velocity = -0.9 * self.y_velocity;
        }

        self.pos.prev_x = self.pos.x;
        self.pos.prev_y= self.pos.y;
        self.pos.x = updated_x_position;
        self.pos.y = updated_y_position;

    }
    pub fn display_ball(&mut self, d: &mut RaylibDrawHandle, alpha: f32) {
        d.draw_circle(self.pos.interpolate_x(alpha), self.pos.interpolate_y(alpha), self.radius, Color::GREEN)
    }
}
