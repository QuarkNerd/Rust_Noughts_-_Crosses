use std::fmt;
use std::collections::HashMap;
use crate::shared_definitions::*;
use crate::players::*;

pub fn play_game(player_one: &mut Player, player_two: &mut Player) {
    // this hashmap needs to use enum
    let mut players_by_symbol = HashMap::new();
    players_by_symbol.insert(PlayerSymbol::X,player_one);
    players_by_symbol.insert(PlayerSymbol::O,player_two); // did it this way so didint have to implement clone on human_player leads to having to make it mutable, is this okay?
    
    let mut board = Board([[None, None, None],
            [None, None, None],
            [None, None, None]]);

    let mut current_symbol = PlayerSymbol::X;
    let mut winner: Option<PlayerSymbol> = None;

    for turn in 1..=9 {
        let current_player = players_by_symbol.get_mut(&current_symbol).unwrap();
        get_player_to_move(&mut board, current_player, current_symbol);
        current_symbol = current_symbol.other();

        if turn > 4  {
            winner = board.get_winner();
            if winner.is_some() { break; }
        }
    }
    inform_player_result(players_by_symbol, winner)
}

fn get_player_to_move(board: &mut Board, player: &mut Player, symbol: PlayerSymbol) {
    let update = GameStatus {
        display_state: board.to_string(),
        minified_state: board.get_minified_state(symbol),
        possible_moves: board.get_possible_moves(),
    };

    let mut was_valid_move_made = false;

    while !was_valid_move_made {
        let next_move = player.make_move(&update);
        was_valid_move_made = board.try_apply_move(symbol, &next_move); // works because copy/clone
    }
}

fn inform_player_result(mut players_by_symbol: HashMap<PlayerSymbol, &mut Player>, winner: Option<PlayerSymbol>) {
    match winner {
        Some(symbol) => {
            players_by_symbol.get_mut(&symbol).unwrap().take_result(Result::Win);
            players_by_symbol.get_mut(&symbol.other()).unwrap().take_result(Result::Lose);
        }
        None => {
            players_by_symbol.get_mut(&PlayerSymbol::X).unwrap().take_result(Result::Draw);
            players_by_symbol.get_mut(&PlayerSymbol::O).unwrap().take_result(Result::Draw);
        }
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

impl PlayerSymbol {
    fn other(&self) -> PlayerSymbol {
        if *self == PlayerSymbol::X { //lookup uses of dereferencing
            PlayerSymbol::O
        } else {
            PlayerSymbol::X
        }
    }
}

struct Board([[Option<PlayerSymbol>; 3] ; 3]);

impl Board {
    fn try_apply_move(&mut self, player: PlayerSymbol, pos: &str) -> bool {
                let pos= pos.parse::<usize>();
                let position = match pos {
                    Ok(input) => input,
                    _ => return false
                };

                if position > 8 { return false; }

                let column = position/3;
                let row = position%3;
        
                if self.0[row][column] != None {
                    return false;
                }
                
                self.0[row][column] = Some(player);
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

    fn get_minified_state(&self, player_symbol: PlayerSymbol) -> String {
        (0..=2).map(|row_num| {
            (0..=2).map(|col_num| {
                return match self.0[row_num][col_num] {
                    Some(x) => {
                        if x == player_symbol {
                           "Y" 
                        } else {
                           "R"
                        }
                    }
                    None => " ",
                };
            }).collect::<Vec<_>>().join("")
        }).collect::<Vec<_>>().join("")
    }

    fn get_possible_moves(&self) -> Vec<String> {
        let mut possible_moves = Vec::new();
        for col_num in 0..=2 {
            for row_num in 0..=2 {
                if self.0[row_num][col_num].is_none() {
                    possible_moves.push((row_num + col_num*3).to_string())
                }
            }
        };
        possible_moves
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_rows = (0..=2).map(|row_num| {
            let row = (0..=2).map(|col_num| {
                return match self.0[row_num][col_num] {
                    Some(x) => x.to_string(),
                    None => (row_num + col_num*3).to_string(),
                };
            }).collect::<Vec<_>>();
            format!("|     |     |     |\n|  {}  |  {}  |  {}  |\n|_____|_____|_____|\n", row[0], row[1], row[2])
        }).collect::<Vec<_>>();

        write!(f, " _________________ \n{}{}{}To make a move, type the number", display_rows[0],display_rows[1],display_rows[2])
    }
}