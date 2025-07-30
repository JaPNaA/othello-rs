use rand::{Rng, rngs::ThreadRng};

use crate::{board::Board, bots::MakeMove};

/// This bot choses a random move
pub struct Bot {
    rng: ThreadRng,
}

impl Bot {
    pub fn new() -> Bot {
        Bot { rng: rand::rng() }
    }
}

impl MakeMove for Bot {
    fn make_move(&mut self, board: &Board, color: bool) -> (i8, i8) {
        let valid_moves = board.get_all_valid_moves(color);

        if valid_moves.is_empty() {
            return (-1, -1);
        }

        valid_moves[self.rng.random_range(0..valid_moves.len())].clone()
    }
}
