use std::fmt;
use crate::definitions::*;

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
    player_one: HumanPlayer,
    player_two: HumanPlayer,
}

impl Game {
    pub fn from(player_one: HumanPlayer, player_two: HumanPlayer) -> Game {
        Game {
            board: [[TicTacToeSquare::Empty, TicTacToeSquare::Empty, TicTacToeSquare::Empty],
            [TicTacToeSquare::Empty, TicTacToeSquare::Empty, TicTacToeSquare::Empty],
            [TicTacToeSquare::Empty, TicTacToeSquare::Empty, TicTacToeSquare::Empty]],
            player_one,
            player_two
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
        display.to_string()
    }
    
    fn make_move(&mut self, player: &str, pos: &str) {
        let pos_split: Vec<&str> = pos.split(" ").collect();
        let column: usize = match pos_split[0] {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => panic!("column invalid")
        };

        let row:usize = pos_split[1].trim().parse::<usize>().unwrap();
        
        let board_square: TicTacToeSquare = match player {
            "X" => TicTacToeSquare::X,
            "O" => TicTacToeSquare::O,
            _ => panic!("player invalid")
        };
        
        self.board[row - 1][column - 1] = board_square;
    }
}
