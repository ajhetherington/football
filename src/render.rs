use raylib::prelude::*;

pub fn render_something() {
    let screen_width = 640;
    let screen_height = 480;
    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Not just Football Manager")
        .build();

    let mut ball_position: Vec<f32> = vec![screen_width as f32 / 2.0, screen_height as f32 / 2.0];
    let ball_radius: f32 = 50.0;
    while !rl.window_should_close() {
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            ball_position[0] = ball_position[0] + 2.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            ball_position[0] = ball_position[0] - 2.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            ball_position[1] = ball_position[1] - 2.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            ball_position[1] = ball_position[1] + 2.0;
        }

        if ball_position[0] < ball_radius {
            ball_position[0] = ball_radius;
        }
        if ball_position[0] > (screen_width as f32) - ball_radius {
            ball_position[0] = (screen_width as f32) - ball_radius;
        }
        if ball_position[1] < ball_radius {
            ball_position[1] = ball_radius;
        }
        if ball_position[1] > (screen_height as f32) - ball_radius {
            ball_position[1] = (screen_height as f32) - ball_radius;
        }


        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world", 12, 12, 20, Color::BLACK);
        d.draw_circle(ball_position[0].floor() as i32, ball_position[1].floor() as i32, ball_radius, Color::MAROON)
        
    }
}
