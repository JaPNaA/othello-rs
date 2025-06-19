use rand::Rng;

use crate::{board::Board, bots::MakeMove};

pub struct Bot {}

impl Bot {
    pub fn new() -> Bot {
        Bot {}
    }
}

impl MakeMove for Bot {
    fn make_move(&self, board: &Board, color: bool) -> (i8, i8) {
        let valid_moves = board.get_all_valid_moves(color);

        if valid_moves.is_empty() {
            return (-1, -1);
        }

        valid_moves[rand::rng().random_range(0..valid_moves.len())].clone()
    }
}
