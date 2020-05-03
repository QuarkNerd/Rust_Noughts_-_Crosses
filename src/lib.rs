use definitions::*;

pub fn run() {
    let mut one = HumanPlayer::new("Player one name:");
    let mut two = HumanPlayer::new("Player two name:");
    // let mut game = normal_game::Game::from(one, two);
    normal_game::play_game(&one, &two);
}

mod normal_game;
mod definitions;