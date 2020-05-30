use players::*;
//cheched_sub
pub fn run() {
    let mut one = Player::HumanPlayer(HumanPlayer::new("Player one name:"));
    let mut two = Player::HumanPlayer(HumanPlayer::new("Player one name:"));
    normal_game::play_game(&mut one, &mut two);
}

mod normal_game;
mod players;
mod utilities;
mod shared_definitions;