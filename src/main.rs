
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

}
