use crate::{board::Board, bots::MakeMove};

/// This bot always chooses the last valid move.
pub struct Bot {}

impl Bot {
    pub fn new() -> Bot {
        Bot {}
    }
}

impl MakeMove for Bot {
    fn make_move(&mut self, board: &Board, color: bool) -> (i8, i8) {
        let valid_moves = board.get_all_valid_moves(color);

        match valid_moves.last() {
            Some(x) => x.clone(),
            None => (-1, -1),
        }
    }
}
