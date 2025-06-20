use rand::{rngs::ThreadRng, seq::SliceRandom};

use crate::{board::Board, bots::MakeMove};

/// This bot choses the move that is closest to an edge
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

        let best_move = moves.into_iter().min_by_key(|m| evaluate_move(m));

        match best_move {
            Some(x) => x,
            None => (-1, -1),
        }
    }
}

fn evaluate_move(m: &(i8, i8)) -> i8 {
    let x_score = if m.0 < 4 { m.0 } else { 7 - m.0 };
    let y_score = if m.1 < 4 { m.1 } else { 7 - m.1 };
    x_score + y_score
}
