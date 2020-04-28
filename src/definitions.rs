use std::io;

pub struct StatusUpdate {
    display_state: String,
    game_in_progress: bool,
}

pub struct HumanPlayer {
    identity: String,
}

impl HumanPlayer {
    pub fn give_update(&self, update: StatusUpdate) -> Option<String> {
        println!("Message for {}", &self.identity);
        println!("{}", update.display_state);
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
    guess
}