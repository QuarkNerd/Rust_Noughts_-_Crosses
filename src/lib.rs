use players::*;

pub fn run() {
    let one = HumanPlayer::new("Player one name:");
    let two = HumanPlayer::new("Player two name:");
    normal_game::play_game(&one, &two);
}

mod normal_game;
mod players;
mod utilities;
mod shared_definitions;