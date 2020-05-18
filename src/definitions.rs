use std::io;
use std::fmt::Display;

pub enum Result {
    Win, Draw, Lose
}

pub struct StatusUpdate {
    pub display_state: String,
}

pub trait Player {
    fn make_move(&self, update: &StatusUpdate) -> String;
    fn take_result(&self, result: Result);
    fn new_game(&self);
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

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    guess = guess.trim().to_string();
    guess
}
