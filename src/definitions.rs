use std::io;

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

impl HumanPlayer {
    pub fn new(prompt : &str) -> HumanPlayer {
        HumanPlayer {
            identity: get_user_input(prompt)
        }
    }
    
    pub fn make_move(&self, update: &StatusUpdate) -> String {
        self.print_msg_start();
        println!("{}", update.display_state);
        self.print_msg_end();
        get_user_input("What is your next move?")
    }

    pub fn take_result(&self, result: Result) {
        self.print_msg_start();
        let msg = match result {
            Result::Win => "You win, well done?.. I guess",
            Result::Draw => "Draw occurred, acceptable",
            Result::Lose => "You Lost, but.... how?",
        };
        println!("{}", msg);
        self.print_msg_end();
    }

    fn print_msg_start(&self) {
        println!("--------Message for {}-------\n", &self.identity);
    }

    fn print_msg_end(&self) {
        println!("----------------------------");
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
