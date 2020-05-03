use std::io;

pub struct StatusUpdate {
    pub display_state: String,
    pub game_in_progress: bool,
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

    pub fn change_name(&mut self, name: String) {
        self.identity = name;
    }

    pub fn give_update(&self, update: &StatusUpdate) -> Option<String> {
        println!("--------Message for {}-------\n", &self.identity);
        println!("{}", update.display_state);
        println!("----------------------------");
        if update.game_in_progress {
            Some(get_user_input("What is your next move?"))
        } else {
            None
        }
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