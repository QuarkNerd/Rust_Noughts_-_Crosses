use std::fmt::Display;
use crate::utilities::*;
use crate::shared_definitions::*;

pub trait Player {
    fn new_game(&self);
    fn make_move(&self, update: &StatusUpdate) -> String;
    fn take_result(&self, result: Result);
}

pub struct HumanPlayer {
    pub identity: String,
}

impl Player for HumanPlayer {
    // don't need to make public because it's implied by use of a trait
    fn new_game(&self) {
        self.print_msg("Get ready for a game!");
    }

    fn make_move(&self, update: &StatusUpdate) -> String {
        self.print_msg(&update.display_state);
        get_user_input("What is your next move?")
    }

    fn take_result(&self, result: Result) {
        let msg = match result {
            Result::Win => "You win, well done?.. I guess",
            Result::Draw => "Draw occurred, acceptable",
            Result::Lose => "You Lost, but.... how?",
        };
        self.print_msg(msg);
    }
}

impl HumanPlayer {
    pub fn new(prompt : &str) -> HumanPlayer {
        HumanPlayer {
            identity: get_user_input(prompt)
        }
    }

    fn print_msg(&self, msg: impl Display) {
        println!("{:-^30}\n{}\n{:-<30}", &self.identity, msg, "");
    }

}

