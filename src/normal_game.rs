use std::fmt;
use std::collections::HashMap;
use crate::definitions::*;

pub fn play_game(player_one: &HumanPlayer, player_two: &HumanPlayer) {
    let mut players_by_symbol = HashMap::new();
    players_by_symbol.insert(PlayerSymbol::X,player_one);
    players_by_symbol.insert(PlayerSymbol::O,player_two); // did it this way so didint have to implement clone on human_player leads to having to make it mutable, is this okay?
    
    let mut board = Board([[None, None, None],
            [None, None, None],
            [None, None, None]]);

    let mut current_symbol = PlayerSymbol::X;
    let mut winner: Option<PlayerSymbol> = None;

    for turn in 1..=9 {
        let current_player = players_by_symbol.get(&current_symbol).unwrap();
        get_player_to_move(&mut board, current_player, current_symbol);
        current_symbol = current_symbol.other();

        if turn > 4  {
            winner = board.get_winner();
            if winner.is_some() { break; }
        }
    }
    match winner {
        Some(player) => println!("{}", player),
        None => println!("peep")
    }
}

fn get_player_to_move(board: &mut Board, player: &HumanPlayer, symbol: PlayerSymbol) {
    let update = StatusUpdate {
        display_state: board.to_string(),
    };

    let mut have_valid_move = false;

    while !have_valid_move {
        let next_move = player.make_move(&update);
        have_valid_move = board.try_apply_move(symbol, &next_move); // works because copy/clone
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum PlayerSymbol {
    X, O
}

impl fmt::Display for PlayerSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let disp = match &self {
            PlayerSymbol::X => "X",
            PlayerSymbol::O => "O"
        };
        write!(f, "{}", disp)
    }
}

// enum Result {
//     Win(PlayerSymbol), Draw
// }

impl PlayerSymbol {
    fn other(&self) -> PlayerSymbol {
        if *self == PlayerSymbol::X { //lookup uses of dereferencing
            PlayerSymbol::O
        } else {
            PlayerSymbol::X
        }
    }
}

pub struct Board([[Option<PlayerSymbol>; 3] ; 3]);

impl Board {
    fn try_apply_move(&mut self, player: PlayerSymbol, pos: &str) -> bool {
                let pos_split: Vec<&str> = pos.split(" ").collect();
        
                let column: usize = match pos_split[0] {
                    "A" | "a" => 1,
                    "B" | "b" => 2,
                    "C" | "c" => 3,
                    _ => return false
                };
                // not using parse because then I wouldneed extra logic to stop panics and to make sure
                // row is in bounds
                let row: usize = match pos_split[1] {
                    "1" => 1,
                    "2" => 2,
                    "3" => 3,
                    _ => return false
                };
        
                if self.0[row - 1][column - 1] != None {
                    return false
                }
                
                self.0[row - 1][column - 1] = Some(player);
                true
            }
    
    fn get_winner(&self) -> Option<PlayerSymbol> {
        let board = self.0;
        
        // Rows and columns
        for coor in 0..3 {
            if (board[coor][0] != None && board[coor][0] == board[coor][1] && board[coor][1] == board[coor][2]) || // row 
                (board[0][coor] != None && board[0][coor] == board[1][coor] && board[1][coor] == board[2][coor]) { // column
                    return board[coor][coor]
            }
        }

        // diagonals
        if (board[1][1] != None) && ( 
            (board[0][0] == board[1][1] && board[1][1] == board[2][2]) || 
            (board[2][0] == board[1][1] && board[1][1] == board[0][2])
        ) { 
            return board[1][1]
        }
        None
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display = "     A     B     C  \n".to_string();
        let blank_line = "        |     |     \n";
        let blank_underlined_line = "   _____|_____|_____\n";
        
        for x in 0..8 {
            match x%3 {
                0 => display.push_str(blank_line),
                1 => {
                    let row_num = x/3;
                    let row: Vec<String> = (0..3).map(|col| {
                        match self.0[row_num][col] {
                            Some(x) => x.to_string(),
                            None => " ".to_string(),
                        } 
                    }).collect();
                    display.push_str(&format!("{}    {}  |  {}  |  {}  \n",row_num + 1 ,row[0], row[1], row[2]))
                },
                2 => display.push_str(blank_underlined_line),
                _ => unreachable!()
            }
        }
        display.push_str(blank_line);
        display.push_str("To make a move, type the letter and number like \"B 3\"");
        write!(f, "{}", display)
    }
}