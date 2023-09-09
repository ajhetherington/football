use crate::window::ScreenSize;
use raylib::core::drawing::RaylibDrawHandle;
use raylib::prelude::{Color, RaylibDraw};

pub struct Pitch {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Pitch {
    pub fn new(screen_size: &ScreenSize) -> Self {
        let x = ((screen_size.width as f32) * 0.1).round() as i32;
        let y = ((screen_size.height as f32) * 0.1).round() as i32;
        let width = ((screen_size.width as f32) * 0.8).round() as i32;
        let height = ((screen_size.height as f32) * 0.8).round() as i32;
        Pitch {
            x,
            y,
            width,
            height,
        }
    }
}

pub fn render_pitch(d: &mut RaylibDrawHandle, pitch: &Pitch) {
    // 10% inset
    let pitch_x = pitch.x;
    let pitch_y = pitch.y;
    let pitch_width = pitch.width;
    let pitch_height = pitch.height;

    {
        // pitch outskirts
        d.draw_rectangle_lines(pitch_x, pitch_y, pitch_width, pitch_height, Color::BLACK)
    }
    {
        // center circle
        let pos_x = ((pitch_width as f32) * 0.5).round() as i32 + pitch_x;
        let pos_y = ((pitch_height as f32) * 0.5).round() as i32 + pitch_y;
        let radius = 30.0;
        d.draw_circle_lines(pos_x, pos_y, radius, Color::BLACK)
    }
    {
        // center line
        let pos_x = ((pitch_width as f32) * 0.5).round() as i32 + pitch_x;
        let start_pos_y = pitch_y;
        let end_pos_y = pitch_y + pitch_height;
        d.draw_line(pos_x, start_pos_y, pos_x, end_pos_y, Color::BLACK)
    }
    {
        // goal lines
        let mut pos_x = pitch.x;
        const GOAL_LENGTH: f32 = 70.0;
        let start_pos_y = (pitch.y as f32) + (pitch.height as f32 / 2.0) - (GOAL_LENGTH / 2.0);
        let end_pos_y = (pitch.y as f32) + (pitch.height as f32 / 2.0) + (GOAL_LENGTH / 2.0);
        d.draw_line(
            pos_x,
            start_pos_y.ceil() as i32,
            pos_x,
            end_pos_y.round() as i32,
            Color::ORANGE,
        );

        pos_x = pitch.x + pitch.width;
        d.draw_line(
            pos_x,
            start_pos_y.round() as i32,
            pos_x,
            end_pos_y.round() as i32,
            Color::ORANGE,
        );
    }
}