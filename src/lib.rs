use definitions::*;

pub fn run() {
    let one = HumanPlayer::new("Player one name:");
    let two = HumanPlayer::new("Player two name:");
    let mut game = normal_game::Game::from(one, two);
    game.play_game();
}

mod normal_game;
mod definitions;