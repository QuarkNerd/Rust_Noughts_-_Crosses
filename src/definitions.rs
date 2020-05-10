use std::io;

pub struct StatusUpdate {
    pub display_state: String,
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
        println!("--------Message for {}-------\n", &self.identity);
        println!("{}", update.display_state);
        println!("----------------------------");
        get_user_input("What is your next move?")

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