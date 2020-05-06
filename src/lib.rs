use definitions::*;

pub fn run() {
    let one = HumanPlayer::new("Player one name:");
    let two = HumanPlayer::new("Player two name:");
    normal_game::play_game(&one, &two);
}

mod normal_game;
mod definitions;