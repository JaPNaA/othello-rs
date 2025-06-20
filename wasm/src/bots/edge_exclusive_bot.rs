use rand::{rngs::ThreadRng, seq::SliceRandom};

use crate::{board::Board, bots::MakeMove};

/// This bot makes random moves unless it can place a piece on the edge
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

        match moves.into_iter().max_by_key(|m| evaluate_move(m)) {
            Some(x) => x,
            None => (-1, -1),
        }
    }
}

fn evaluate_move(m: &(i8, i8)) -> i8 {
    let x_score = if m.0 == 0 || m.0 == 7 { 1 } else { 0 };
    let y_score = if m.1 == 0 || m.1 == 7 { 1 } else { 0 };
    x_score + y_score
}
