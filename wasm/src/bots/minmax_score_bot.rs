use crate::{board::Board, bots::MakeMove};

/// This bot picks the move that gives it the most score after the best opponent move
pub struct Bot {}

impl Bot {
    pub fn new() -> Bot {
        Bot {}
    }
}

impl MakeMove for Bot {
    fn make_move(&mut self, board: &Board, color: bool) -> (i8, i8) {
        let moves = board.get_all_valid_moves(color);

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
    let opponent_moves = future.get_all_valid_moves(!color);

    match opponent_moves
        .into_iter()
        .map(|op_m| {
            let mut future2 = future.clone();
            future2.try_place_chip(op_m.0, op_m.1, !color);
            heuristic_score(&future2, color)
        })
        .min()
    {
        None => heuristic_score(&future, color), // to be more accurate, this should be the score of the best next move
        Some(x) => x,
    }
}

fn heuristic_score(board: &Board, color: bool) -> u16 {
    board.count_pieces(color)
}
