use crate::{
    board::Board,
    bots::{MakeMove, deep_heuristic_bot},
};

/// This bot maximizes a heuristic score (guessed by the developer)
/// by doing a 5-deep minmax search
pub struct Bot {}

impl Bot {
    pub fn new() -> Bot {
        Bot {}
    }
}

const NEG_INF_SCORE: f32 = f32::MIN;
const POS_INF_SCORE: f32 = f32::MAX;

impl MakeMove for Bot {
    fn make_move(&mut self, board: &Board, color: bool) -> (i8, i8) {
        let moves = board.get_all_valid_moves(color).into_iter();

        let moves = moves;

        let mut best_score = NEG_INF_SCORE;
        let mut best_move = (-1, -1);

        for m in moves {
            let mut future = board.clone();
            future.try_place_chip(m.0, m.1, color);
            let result = -deep_heuristic_bot::evaluate_board(
                &future,
                color,
                !color,
                4,
                NEG_INF_SCORE,
                POS_INF_SCORE,
            );

            if result > best_score {
                best_move = m;
                best_score = result;
            }
        }

        best_move
    }
}
