use std::collections::hash_map::Entry;

use std::fmt::Display;
use std::collections::HashMap;
use crate::utilities::*;
use rand::distributions::WeightedIndex;

use rand::prelude::*;
use crate::shared_definitions::*;
use std::u64;

pub trait Player {
    fn make_move(&mut self, update: &GameStatus) -> String;
    fn take_result(&mut self, result: Result);
}

pub struct HumanPlayer {
    identity: String,
}

impl Player for HumanPlayer {
    // don't need to make public because it's implied by use of a trait
    fn make_move(&mut self, update: &GameStatus) -> String {
        self.print_msg(&update.display_state);
        get_user_input("What is your next move?")
    }

    fn take_result(&mut self, result: Result) {
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

// could store as a HashMap but then wold have to extract the keys and values on every weighted_pick
struct Strategy {
    moves: Vec<String>,
    weights: Vec<u64>
}

impl Strategy {
    pub fn create_fresh(moves: Vec<String>, weight: u64) -> Strategy {
        // doing these two fields the other way around causes an error, because moves.len()
        // borrows the moves the value of which has had it ownership change
        Strategy {
            weights : vec![weight; moves.len()],
            moves,
        }
    }

    pub fn weighted_pick(&self) -> String {
        let dist = WeightedIndex::new(&(self.weights)).unwrap();
        let mut rng = thread_rng();
        self.moves[dist.sample(&mut rng)].clone()
    }

    pub fn update_weight(&mut self, move_string: &String, change: i64 ) {
        let move_index = self.moves.iter().position(|s| s == move_string).unwrap_or_else(|| {
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

const STARTING_WEIGHT: u64 = 15;
const WIN_CHANGE:i64 = 5;
const DRAW_CHANGE:i64 = 0;
const LOSE_CHANGE: i64 = -2;

pub struct ComputerLearner {
    // used to use static str but that cannot be calculated at runtime as state might be
    strategy_by_state: HashMap<String, Strategy>,
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

impl Player for ComputerLearner {
    fn make_move(&mut self, update: &GameStatus) -> String {
        // clone removed "lifetime conflicting requriements error", not sure why it occurs,
        // after which the key to hashmap was changed to use static str
        let current_strategy: &Strategy = match self.strategy_by_state.entry(update.minified_state.clone()) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(Strategy::create_fresh(update.possible_moves.clone(), STARTING_WEIGHT))
        };

        let selected_move = current_strategy.weighted_pick();
        self.current_game_history.push(GameStep {game_state: update.minified_state.clone(), move_made:selected_move.clone()});

        selected_move
    }
    fn take_result(&mut self, result: Result) {
        let change = match result {
            Result::Win => WIN_CHANGE,
            Result::Draw => DRAW_CHANGE,
            Result::Lose => LOSE_CHANGE,
        };

        // can iterate over vectors by default, that caused errors as that gives ownership of elemtnst to 'step'
        // using iter_mut() would return mutable references.
        for step in self.current_game_history.iter() {
            self.strategy_by_state.get_mut(&step.game_state).unwrap().update_weight(&step.move_made, change);
        }

        self.current_game_history = Vec::new();
    }
}