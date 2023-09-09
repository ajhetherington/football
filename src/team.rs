use crate::player::Player;


#[derive(Debug)]
pub enum TeamSide {
    Home,
    Away
}

#[derive(Debug)]
pub struct Team<'a> {
    pub name: String,
    pub players: [&'a Player; 5], // Array of references to 5 player objects
    pub side: TeamSide,
}

impl<'a> Team<'a> {
    pub fn new(name: String, players: [&'a Player; 5], team_side: TeamSide) -> Self {
            Team { name, players, side: team_side }
    }

    pub fn generate_players() -> [Player; 5] {
        let player_names = ["player1", "player2", "player3", "player4", "player5"];
        let players = [
            Player::new(player_names[0].to_string(), 1).unwrap(),
            Player::new(player_names[1].to_string(), 2).unwrap(),
            Player::new(player_names[2].to_string(), 3).unwrap(),
            Player::new(player_names[3].to_string(), 4).unwrap(),
            Player::new(player_names[4].to_string(), 5).unwrap(),
        ];
        players
    }
}
