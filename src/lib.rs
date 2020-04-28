use std::io;

pub fn run() {
    let mut a = normal_game::Game::new();
    let mut player = "X";

    loop {
        let pos = &get_user_input("Move  ");
        a.make_move(player, pos);
        println!("{}", a.get_state_display());
        player = if player == "X" {
            "O"
        } else {
            "X"
        }
    }

}

mod normal_game;
mod definitions;