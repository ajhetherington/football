use crate::ball::Ball;
use crate::gameobject::GameObject;
use crate::pitch::Pitch;
use crate::player::Player;
use macroquad::color::Color;
use macroquad::input::is_key_down;
use macroquad::miniquad::KeyCode;
use macroquad::prelude::QuadGl;
use macroquad::shapes::draw_circle;

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
    color: Color,
}

impl<'a> VisiblePlayer<'a> {
    pub fn new(player: &'a Player, x: f32, y: f32, color: Color) -> Self {
        return VisiblePlayer {
            player,
            object: GameObject::new(x, y, 8.0, 2.0, 0.8),
            color,
            movable: false,
        };
    }
    pub fn to_movable(&mut self) {
        self.movable = true;
    }
    pub fn is_movable(self) -> bool {
        self.movable
    }

    pub fn new_handle_user_movement(&mut self, _qgl: &mut QuadGl, dt: f32) {
        let movement_force = (self.player.physicals.strength as f32) * 10.0;
        if !(self.movable) {
            return;
        }
        if is_key_down(KeyCode::Right) {
            self.object.apply_force(movement_force, 0.0, dt);
        }

        if is_key_down(KeyCode::Left) {
            self.object.apply_force(-movement_force, 0.0, dt);
        }
        if is_key_down(KeyCode::Up) {
            self.object.apply_force(0.0, -movement_force, dt);
        }
        if is_key_down(KeyCode::Down) {
            self.object.apply_force(0.0, movement_force, dt);
        }
    }

    pub fn handle_kick_ball(&mut self, ball: &mut Ball, x_dir: f32, y_dir: f32, dt: f32) {
        // ok, so first check whether we can hit the ball
        // then just apply a big force in a particular direction
        if !(self.movable) {
            return;
        }
        let (x_partial, y_partial) = normalize(x_dir, y_dir);
        let mut force = self.player.physicals.strength as f32;
        force *= 20.0;
        force += 100.0;
        let accuracy = self.player.skills.technique as f32;

        ball.object
            .apply_force(force * x_partial, force * y_partial, dt)
    }

    pub fn handle_physics(&mut self, pitch: &Pitch, dt: f32) {
        self.object.apply_friction(dt);
        self.object.update_position(pitch, dt);
    }

    pub fn new_render(&self, _qgl: &mut QuadGl, alpha: f32) {
        draw_circle(
            self.object.pos.interpolate_x(alpha),
            self.object.pos.interpolate_y(alpha),
            self.object.radius,
            // hack, idk why deref not working
            self.color.into(),
        )
    }
}

fn normalize(x: f32, y: f32) -> (f32, f32) {
    let norm: f32 = (x.powf(2.0) + y.powf(2.0)).sqrt();
    (x / norm, y / norm)
}
