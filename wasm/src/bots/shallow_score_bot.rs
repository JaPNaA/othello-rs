use rand::{rngs::ThreadRng, seq::SliceRandom};

use crate::{board::Board, bots::MakeMove};

/// This bot chooses the move that flips the most opponent pieces in a single move
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
        let mut moves = board.get_all_valid_moves(color);
        moves.shuffle(&mut self.rng);

        match moves
            .into_iter()
            .max_by_key(|m| evaluate_move(m, &board, color))
        {
            Some(x) => x,
            None => (-1, -1),
        }
    }
}

fn evaluate_move(m: &(i8, i8), board: &Board, color: bool) -> u16 {
    let mut future = board.clone();
    future.try_place_chip(m.0, m.1, color);
    future.count_pieces(color)
}
