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
        let mut valid_moves = Vec::new();

        for y in 0..8 {
            for x in 0..8 {
                if board.is_valid_move(x, y, color) {
                    valid_moves.push((x, y));
                }
            }
        }

        if valid_moves.is_empty() {
            return (0, 0);
        }

        valid_moves[rand::rng().random_range(0..valid_moves.len())].clone()
    }
}
