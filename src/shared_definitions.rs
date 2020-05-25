pub enum Result {
    Win, Draw, Lose
}

pub struct GameStatus {
    pub display_state: String,
    pub minified_state: String,
    pub possible_moves: Vec<String>
}
