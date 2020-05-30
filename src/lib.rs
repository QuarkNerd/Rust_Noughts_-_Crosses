use players::*;
//cheched_sub
pub fn run() {
    let mut one = HumanPlayer::new("Player one name:");
    let mut two = HumanPlayer::new("Player two name:");
    normal_game::play_game(&mut one, &mut two);
}

struct pop {
    fizz: u32
}

mod normal_game;
mod players;
mod utilities;
mod shared_definitions;