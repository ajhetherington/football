use std::f32::consts::E;
use crate::position::Position;
use crate::pitch::Pitch;

#[derive(Debug, Copy, Clone)]
pub struct GameObject {
    pub pos: Position,
    pub x_velocity: f32,
    pub y_velocity: f32,
    pub radius: f32,
    pub mass: f32,
    pub friction: f32,
}

const WALL_FRICTION: f32 = 0.99;
const ELASTICITY: f32 = 0.9;

impl GameObject {
    pub fn apply_force(&mut self, force_x: f32, force_y: f32, dt: f32) {
        let accel_x = force_x / self.mass;
        let accel_y = force_y / self.mass;
        self.x_velocity += dt * accel_x;
        self.y_velocity += dt * accel_y;
    }
    pub fn apply_friction(&mut self, dt: f32) {
        // v(t+dt)=v(t)× exp(−friction×dt)
        let decay_factor = E.powf(-self.friction * dt);
        self.x_velocity *= decay_factor;
        self.y_velocity *= decay_factor;
        if self.x_velocity.abs() < 0.001 {
            self.x_velocity = 0.0
        }
        if self.y_velocity.abs() < 0.001 {
            self.y_velocity = 0.0
        }
    }

    fn check_wall_collisions(&mut self, mut updated_x_position: f32, mut updated_y_position: f32, pitch: &Pitch ) {
        // checking
        if updated_x_position <= (pitch.x as f32) + self.radius {
            updated_x_position = (pitch.x as f32) + self.radius;
            self.x_velocity = -1.0 * ELASTICITY * WALL_FRICTION * self.x_velocity;
            self.y_velocity = WALL_FRICTION * self.y_velocity;
        } else if updated_x_position >= ((pitch.x + pitch.width) as f32) - self.radius {
            updated_x_position = ((pitch.x + pitch.width) as f32) - self.radius;
            self.x_velocity = -1.0 * ELASTICITY * WALL_FRICTION * self.x_velocity;
            self.y_velocity = WALL_FRICTION * self.y_velocity;
        }

        if updated_y_position <= (pitch.y as f32) + self.radius {
            updated_y_position = (pitch.y as f32) + self.radius;
            self.y_velocity = -1.0 * ELASTICITY * WALL_FRICTION * self.y_velocity;
            self.x_velocity = WALL_FRICTION * self.x_velocity;
        } else if updated_y_position >= ((pitch.y + pitch.height) as f32) - self.radius {
            updated_y_position = ((pitch.y + pitch.height) as f32) - self.radius;
            self.y_velocity = -1.0 * ELASTICITY * WALL_FRICTION * self.y_velocity;
            self.x_velocity = WALL_FRICTION * self.x_velocity;
        }

    }

    pub fn update_position(&mut self, pitch: &Pitch, dt: f32) {
        let mut updated_x_position = self.pos.x + (self.x_velocity * dt);
        let mut updated_y_position = self.pos.y + (self.y_velocity * dt);

        self.check_wall_collisions(updated_x_position, updated_y_position, pitch);

        self.pos.prev_x = self.pos.x;
        self.pos.prev_y = self.pos.y;
        self.pos.x = updated_x_position;
        self.pos.y = updated_y_position;
    }
}
