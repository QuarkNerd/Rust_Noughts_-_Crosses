use std::fmt;

pub fn run() {
    let a = TicTacToeGame::new();
    println!("{}", a.get_state_display())

}

pub enum TicTacToeSquare {
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

pub struct TicTacToeGame {
    board: [[TicTacToeSquare; 3] ; 3], 
}

impl TicTacToeGame {
    pub fn new() -> TicTacToeGame {
        TicTacToeGame {
            board: [[TicTacToeSquare::Empty, TicTacToeSquare::Empty, TicTacToeSquare::Empty],
                    [TicTacToeSquare::Empty, TicTacToeSquare::Empty, TicTacToeSquare::Empty],
                    [TicTacToeSquare::Empty, TicTacToeSquare::Empty, TicTacToeSquare::Empty]]
        }
    }

    pub fn get_state_display(&self) -> String {
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
}