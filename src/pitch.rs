use crate::ball::Ball;
use crate::window::ScreenSize;
use raylib::core::drawing::RaylibDrawHandle;
use raylib::core::math::Rectangle;
use raylib::ffi::Vector2;
use raylib::prelude::{Color, RaylibDraw};

const GOAL_LENGTH: f32 = 70.0;
const RECT_WIDTH: f32 = 6.0;

pub struct Pitch {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    left_goal: Rectangle,
    right_goal: Rectangle,
}

#[derive(Debug)]
pub enum Goal {
    HOME,
    AWAY,
}

impl Pitch {
    pub fn new(screen_size: &ScreenSize) -> Self {
        let x = ((screen_size.width as f32) * 0.1).round() as i32;
        let y = ((screen_size.height as f32) * 0.1).round() as i32;
        let width = ((screen_size.width as f32) * 0.8).round() as i32;
        let height = ((screen_size.height as f32) * 0.8).round() as i32;
        let (left_goal, right_goal) = Pitch::make_rectangles(x, y, width, height);
        Pitch {
            x,
            y,
            width,
            height,
            left_goal,
            right_goal,
        }
    }

    fn make_rectangles(x: i32, y: i32, width: i32, height: i32) -> (Rectangle, Rectangle) {
        let rect1 = Rectangle {
            x: x as f32,
            y: (y as f32) + (height as f32 / 2.0) - (GOAL_LENGTH / 2.0),
            width: RECT_WIDTH,
            height: GOAL_LENGTH,
        };
        let rect2 = Rectangle {
            x: ((x + width) as f32) - RECT_WIDTH,
            y: (y as f32) + (height as f32 / 2.0) - (GOAL_LENGTH / 2.0),
            width: RECT_WIDTH,
            height: GOAL_LENGTH,
        };

        (rect1, rect2)
    }

    pub fn check_goal(&self, ball: &Ball) -> Option<Goal> {
        let center = Vector2 {
            x: ball.object.pos.x,
            y: ball.object.pos.y,
        };
        if self
            .left_goal
            .check_collision_circle_rec(center, ball.object.radius)
        {
            Some(Goal::AWAY)
        } else if self
            .right_goal
            .check_collision_circle_rec(center, ball.object.radius)
        {
            Some(Goal::HOME)
        } else {
            None
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
    {
        d.draw_rectangle(pitch.left_goal.x as i32, pitch.left_goal.y as i32, pitch.left_goal.width as i32, pitch.left_goal.height as i32, Color::PINK);
        d.draw_rectangle(pitch.right_goal.x as i32, pitch.right_goal.y as i32, pitch.right_goal.width as i32, pitch.right_goal.height as i32, Color::PINK);
    }
}
