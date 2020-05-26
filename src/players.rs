use std::fmt::Display;
use std::collections::HashMap;
use crate::utilities::*;
use rand::distributions::WeightedIndex;

use rand::prelude::*;
use crate::shared_definitions::*;
use std::u64;

pub trait Player {
    fn make_move(&self, update: &GameStatus) -> String;
    fn take_result(&self, result: Result);
}

pub struct HumanPlayer {
    identity: String,
}

impl Player for HumanPlayer {
    // don't need to make public because it's implied by use of a trait
    fn make_move(&self, update: &GameStatus) -> String {
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

struct GameStep {
    game_state: String,
    move_made: String,
}

struct Strategy {
    moves: Vec<String>,
    weights: Vec<u64>
}

impl Strategy {
    pub fn from(moves: Vec<String>, weights: Vec<u64>) -> Strategy {
        Strategy {
            moves,
            weights,
        }
    }

    pub fn weighted_pick(&self) -> String {
        let dist = WeightedIndex::new(&(self.weights)).unwrap();
        let mut rng = thread_rng();
        self.moves[dist.sample(&mut rng)].clone()
    }

    pub fn update_weight(&mut self, move_string: String, change: i64 ) {
        let move_index = self.moves.iter().position(|s| s == &move_string).unwrap_or_else(|| {
            panic!("update_weight called with unrecognised move");
            }   
        );

        let result = if change < 0 {
            self.weights[move_index].checked_sub(change.abs() as u64)
        } else {
            self.weights[move_index].checked_add(change as u64)
        };

        self.weights[move_index] = result.unwrap_or_else(|| {
            if change < 0 {
                return 0
            } else {
                return u64::MAX
            };
        })
    }
}

pub struct ComputerLearner {
    strategy_by_state: HashMap<&'static str, Strategy>,
    current_game_history: Vec<GameStep>, // decided against HashMap because some games may allow repeating step
}

impl ComputerLearner {
    pub fn new() -> ComputerLearner {
        ComputerLearner {
            strategy_by_state: HashMap::new(),
            current_game_history: Vec::new(),
        }
    }
}