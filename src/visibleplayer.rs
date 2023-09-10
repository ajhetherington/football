use crate::gameobject::GameObject;
use crate::player::Player;
use crate::pitch::Pitch;
use raylib::core::drawing::RaylibDrawHandle;
use raylib::prelude::*;

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


    pub fn handle_user_input(&mut self, rl: &mut RaylibHandle, dt: f32) {
        if !(self.movable) {
            return
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.object.apply_force(8.0, 0.0, dt);
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.object.apply_force(-8.0, 0.0, dt);
        }
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            self.object.apply_force(0.0, -8.0, dt);
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            self.object.apply_force(0.0, 8.0, dt);
        }

    }

    pub fn handle_physics(&mut self, pitch: &Pitch, dt: f32) {
        self.object.apply_friction(dt);
        self.object.update_position(pitch, dt);
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, alpha: f32) {
        d.draw_circle(
            self.object.pos.interpolate_x(alpha),
            self.object.pos.interpolate_y(alpha),
            self.object.radius,
            self.color,
        )
    }
}
