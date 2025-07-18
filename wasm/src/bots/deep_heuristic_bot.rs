use crate::{board::Board, bots::MakeMove};

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
            let result = evaluate_board(&future, color, !color, 4, NEG_INF_SCORE, POS_INF_SCORE);

            if result > best_score {
                best_move = m;
                best_score = result;
            }
        }

        best_move
    }
}

fn evaluate_board(
    board: &Board,
    color: bool,
    next_turn_color: bool,
    depth: u16,
    alpha: f32,
    beta: f32,
) -> f32 {
    if depth == 0 {
        return heuristic_score(board, color);
    }

    let next_moves = board.get_all_valid_moves(next_turn_color);

    if next_moves.is_empty() {
        return evaluate_board(&board, color, !next_turn_color, depth - 1, alpha, beta);
    }

    let next_moves = next_moves.into_iter();

    if next_turn_color == color {
        // our move, find max
        let mut max = alpha;

        for m in next_moves {
            let mut future_board = board.clone();
            future_board.try_place_chip(m.0, m.1, next_turn_color);
            // this branch is (result or worse)
            let result =
                evaluate_board(&future_board, color, !next_turn_color, depth - 1, max, beta);
            // alphabeta = result.1;

            if result >= max {
                max = result;

                if max >= beta {
                    // opponent has better options than this branch, stop search
                    return max;
                }
            }
        }

        max
    } else {
        // their move, find min
        let mut min = beta;

        for m in next_moves {
            let mut future_board = board.clone();
            future_board.try_place_chip(m.0, m.1, next_turn_color);
            // this branch is (result or worse)
            let result = evaluate_board(
                &future_board,
                color,
                !next_turn_color,
                depth - 1,
                alpha,
                min,
            );

            if result < min {
                min = result;

                if min <= alpha {
                    // we have better options than this branch, stop search
                    return min;
                }
            }
        }

        min
    }
}

fn heuristic_score(board: &Board, color: bool) -> f32 {
    let score = f32::from(board.count_pieces(color));
    let opponent_score = f32::from(board.count_pieces(!color));
    let remaining_percent = 1f32 - (score + opponent_score) / 64f32;

    let mut edge_score = 0f32;
    let mut opponent_edge_score = 0f32;

    let mut corner_score = 0f32;
    let mut opponent_corner_score = 0f32;

    // counts the number of edge pieces for the opponent
    // skips the edges
    for i in 1..7 {
        // top
        if board.is_occupied(i, 0) {
            if board.get_color(i, 0) == color {
                edge_score += 1f32;
            } else {
                opponent_edge_score += 1f32;
            }
        }

        // bottom
        if board.is_occupied(i, 7) {
            if board.get_color(i, 7) == color {
                edge_score += 1f32;
            } else {
                opponent_edge_score += 1f32;
            }
        }

        // left
        if board.is_occupied(0, i) {
            if board.get_color(0, i) == color {
                edge_score += 1f32;
            } else {
                opponent_edge_score += 1f32;
            }
        }

        // right
        if board.is_occupied(7, i) {
            if board.get_color(7, i) == color {
                edge_score += 1f32;
            } else {
                opponent_edge_score += 1f32;
            }
        }
    }

    // check four corners
    if board.is_occupied(0, 0) {
        if board.get_color(0, 0) == color {
            corner_score += 1f32;
        } else {
            opponent_corner_score += 1f32;
        }
    }

    if board.is_occupied(7, 0) {
        if board.get_color(7, 0) == color {
            corner_score += 1f32;
        } else {
            opponent_corner_score += 1f32;
        }
    }

    if board.is_occupied(0, 7) {
        if board.get_color(0, 7) == color {
            corner_score += 1f32;
        } else {
            opponent_corner_score += 1f32;
        }
    }

    if board.is_occupied(7, 7) {
        if board.get_color(7, 7) == color {
            corner_score += 1f32;
        } else {
            opponent_corner_score += 1f32;
        }
    }

    (score - opponent_score)
        + ((edge_score - opponent_edge_score) * 3f32
            + (corner_score - opponent_corner_score) * 10f32)
            * (1f32 + remaining_percent) // edges and corners are strong, but don't matter as much in the end game
}
