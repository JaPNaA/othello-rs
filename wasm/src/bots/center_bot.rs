use crate::{board::Board, bots::MakeMove};

/// This bot choses the move that is closest to an edge
pub struct Bot {}

impl Bot {
    pub fn new() -> Bot {
        Bot {}
    }
}

impl MakeMove for Bot {
    fn make_move(&mut self, board: &Board, color: bool) -> (i8, i8) {
        let best_move = board
            .get_all_valid_moves(color)
            .into_iter()
            .max_by_key(|m| evaluate_move(m));

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
