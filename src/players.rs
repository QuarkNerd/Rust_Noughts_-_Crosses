use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fmt;
use std::u64;

use serde::{Serialize, Deserialize};
// use serde::de::DeserializeOwned;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

use crate::utilities::*;
use crate::shared_definitions::*;

const STRATEGY_FOLDER: &str  = r"player_strategy";

fn get_strategy_file_path(filename: &str) -> PathBuf {
        let mut path = PathBuf::new();
        path.push(STRATEGY_FOLDER);
        path.push(filename);
        path
    }

trait PlayerTrait {
    fn make_move(&mut self, update: &GameStatus) -> String;
    fn take_result(&mut self, result: Result);
}

// cant accept generics in the play game function because Hashmap wants values to be same type.
pub enum Player {
    HumanPlayer(HumanPlayer),
    ComputerLearner(ComputerLearner)
}

// match and extract is repeated because cant return a generic type wityhout having the generic come in at enum creation
impl Player {
    pub fn make_move(&mut self, update: &GameStatus) -> String {
        match self {
            Player::HumanPlayer(x)  => return x.make_move(update),
            Player::ComputerLearner(x) => return x.make_move(update)
        }
    }

    pub fn take_result(&mut self, result: Result) {
        match self {
            Player::HumanPlayer(x)  => x.take_result(result),
            Player::ComputerLearner(x) => x.take_result(result)
        };
    }
}

pub struct HumanPlayer {
    identity: String,
}

impl PlayerTrait for HumanPlayer {
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

    fn print_msg(&self, msg: impl fmt::Display) {
        println!("{:-^30}\n{}\n{:-<30}", &self.identity, msg, "");
    }
}

struct GameStep {
    game_state: String,
    move_made: String,
}

// could store as a HashMap but then wold have to extract the keys and values on every weighted_pick
#[derive(Serialize, Deserialize)]
pub struct Strategy {
    pub moves: Vec<String>,
    pub weights: Vec<u64>
}

impl fmt::Debug for Strategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Strategy")
            .field("moves", &self.moves)
            .field("weights", &self.weights)
            .finish()
    }
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

    pub fn reset_weight(&mut self, weight: u64) {
        self.weights = vec![weight; self.moves.len()]
    }

    pub fn is_empty(&mut self) -> bool {
        for x in &self.weights {
            if x != &0 {
                return false;
            }
        };
        true
    }

    //not the best choice as it will choose just one of two if two are equal
    pub fn get_highest_weighted_choice(&self) -> String {
        let mut iter = self.weights.iter().enumerate();
        let init = iter.next().unwrap();
        let highest = iter.try_fold(init, |acc, x| {
        let max = if x.1 > acc.1 {
            x
        } else {
             acc
        };
        Some(max)
        });

        let index_of_highest = highest.unwrap().0;

        self.moves[index_of_highest].clone()
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
    is_learning: bool,
}

impl ComputerLearner {
    pub fn new(is_learning: bool) -> ComputerLearner {
        ComputerLearner {
            strategy_by_state: HashMap::new(),
            current_game_history: Vec::new(),
            is_learning,
        }
    }

    pub fn save(&self, filename: &str) {
        let path = get_strategy_file_path(filename);
        save_with_relative_path(&self.strategy_by_state, path);
    }

    // let a: Strategy = open_with_relative_path::<Strategy>(file);
    pub fn load(filename: &str, is_learning: bool) -> ComputerLearner {
        let path = get_strategy_file_path(filename);
        let a : HashMap<String, Strategy> = open_with_relative_path(path);
        ComputerLearner {
            strategy_by_state: a,//open_with_relative_path(path),
            current_game_history: Vec::new(),
            is_learning,
        }
    }
}

impl PlayerTrait for ComputerLearner {
    fn make_move(&mut self, update: &GameStatus) -> String {
        // clone removed "lifetime conflicting requriements error", 
        let current_strategy: &mut Strategy = match self.strategy_by_state.entry(update.minified_state.clone()) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(Strategy::create_fresh(update.possible_moves.clone(), STARTING_WEIGHT))
        };

        if current_strategy.is_empty() {
            current_strategy.reset_weight(STARTING_WEIGHT);
        }
        
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
        
        if self.is_learning {
            for step in self.current_game_history.iter() {
                self.strategy_by_state.get_mut::<str>(&step.game_state).unwrap().update_weight(&step.move_made, change);
            }
            
            self.current_game_history = Vec::new();
        }
    }
}

// a computer player will load a strategy and only chooses the best option at each stage
pub struct ComputerPlayer {
    strategy_by_state: HashMap<String, String>,
    results_tracking: HashMap<Result, u32>
}

impl ComputerPlayer {
    pub fn load(filename: &str) -> ComputerPlayer {
        let path = get_strategy_file_path(filename);
        let loaded_strategy_by_state : HashMap<String, Strategy> = open_with_relative_path(path);
        
        fn key_value_mapper(initial : (&String, &Strategy)) -> (String, String) {
            (initial.0.clone(), initial.1.get_highest_weighted_choice())
        }

        let results_tracking: HashMap<Result, u32> =
            [
                (Result::Lose, 0 as u32),
                (Result::Draw, 0 as u32),
                (Result::Win, 0 as u32),
                ]
            .iter().cloned().collect();

        ComputerPlayer {
            strategy_by_state: map_a_hash_map(&loaded_strategy_by_state, key_value_mapper),
            results_tracking
        }
    }
    
}

impl PlayerTrait for ComputerPlayer {
    fn make_move(&mut self, update: &GameStatus) -> String {
        self.strategy_by_state[&update.minified_state].clone()
    }
    
    fn take_result(&mut self, result: Result) {
        *self.results_tracking.get_mut(&result).unwrap() += 1;
    }
}