use std::fmt;
use std::collections::HashMap;
use crate::definitions::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum TicTacToeSquare {
    X, O, Empty
}

impl fmt::Display for TicTacToeSquare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let disp = match &self {
            TicTacToeSquare::X => "X",
            TicTacToeSquare::O => "O",
            TicTacToeSquare::Empty => " ",
        };
        write!(f, "{}", disp)
    }
}

pub struct Game {
    board: [[TicTacToeSquare; 3] ; 3],
    players: HashMap<TicTacToeSquare, HumanPlayer>,
    was_previous_move_invalid: bool
}

impl Game {
    pub fn from(player_one: HumanPlayer, player_two: HumanPlayer) -> Game {
        let mut new_game = Game {
            board: [[TicTacToeSquare::Empty, TicTacToeSquare::Empty, TicTacToeSquare::Empty],
            [TicTacToeSquare::Empty, TicTacToeSquare::Empty, TicTacToeSquare::Empty],
            [TicTacToeSquare::Empty, TicTacToeSquare::Empty, TicTacToeSquare::Empty]],
            players: HashMap::new(),
            was_previous_move_invalid: false,
        };
        new_game.players.insert(TicTacToeSquare::X,player_one);
        new_game.players.insert(TicTacToeSquare::O,player_two);
        new_game
    }
    
    pub fn play_game(&mut self) {
        let mut current_symbol = TicTacToeSquare::X;

        loop {
            let update = StatusUpdate {
                display_state: (&self).get_state_display(),
                game_in_progress: true,
            };


            let next_move = &self.players.get(&current_symbol).unwrap().give_update(update).unwrap();
            self.was_previous_move_invalid = !(self).make_move(current_symbol, next_move);
            
            if !self.was_previous_move_invalid {
                current_symbol = if current_symbol == TicTacToeSquare::X {
                    TicTacToeSquare::O
                } else {
                    TicTacToeSquare::X
                }
            }
        }
    }

    fn get_state_display(&self) -> String {
        let mut display = "     A     B     C  \n".to_string();
        let blank_line = "        |     |     \n";
        let blank_underlined_line = "   _____|_____|_____\n";
        
        for x in 0..8 {
            match x%3 {
                0 => display.push_str(blank_line),
                1 => {
                    let row_num = x/3;
                    let row = &self.board[row_num];
                    display.push_str(&format!("{}    {}  |  {}  |  {}  \n",row_num + 1 ,row[0], row[1], row[2]))
                },
                2 => display.push_str(blank_underlined_line),
                _ => unreachable!()
            }
        }
        display.push_str(blank_line);
        display.push_str("To make a move, type the letter and number like \"B 3\"");
        if self.was_previous_move_invalid {
            display.push_str("\nYour previous move was valid!");
        }
        display.to_string()
    }

    // returns whether or not move was valid
    fn make_move(&mut self, player: TicTacToeSquare, pos: &str) -> bool {
        let pos_split: Vec<&str> = pos.split(" ").collect();

        let column: usize = match pos_split[0] {
            "A" | "a" => 1,
            "B" | "b" => 2,
            "C" | "c" => 3,
            _ => return false
        };
        // not using parse because then I need extra logic to stop panics and to make sure
        // row is in bounds
        let row: usize = match pos_split[1] {
            "1" => 1,
            "2" => 2,
            "3" => 3,
            _ => return false
        };

        if self.board[row - 1][column - 1] != TicTacToeSquare::Empty {
            return false
        }
        
        self.board[row - 1][column - 1] = player;
        true
    }
}
