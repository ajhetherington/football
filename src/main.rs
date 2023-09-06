
mod player;
mod team;
use player::Player;
use team::Team;
mod render;
use render::render_something;


fn get_player_ref<'a>(players: &'a [Player; 5]) -> [&'a Player; 5] {
    [&players[0], &players[0], &players[0], &players[0], &players[0]]
}

fn main() {
    for _ in 1..20 {
        let me = Player::new(String::from("alex"), 10).unwrap();
        println!("This is the player {:?}", me);
    }

    let untied_players = Team::generate_players();

    let dyslexia_untied = Team::new(String::from("Dyslexia Untied"),  get_player_ref(&untied_players));
    
    println!("This is the team {:?}", dyslexia_untied);

    render_something()

    // let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        // .expect("Failed to create GLFW window.");

    // window.set_key_polling(true);
    // window.make_current();

    // while !window.should_close() {
        // glfw.poll_events();
        // for (_, event) in glfw::flush_messages(&events) {
            // handle_window_event(&mut window, event);
        // }
    // }
}
