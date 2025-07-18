use crate::{board::Board, bots::MakeMove};

/// This bot maximizes score by doing a 5-deep minmax search
pub struct Bot {}

impl Bot {
    pub fn new() -> Bot {
        Bot {}
    }
}

const NEG_INF_SCORE: u16 = 0;
const POS_INF_SCORE: u16 = 64;

impl MakeMove for Bot {
    fn make_move(&mut self, board: &Board, color: bool) -> (i8, i8) {
        let moves = board.get_all_valid_moves(color).into_iter();

        let moves = moves;

        let mut best_score = 0;
        let mut best_move = (-1, -1);

        for m in moves {
            let mut future = board.clone();
            future.try_place_chip(m.0, m.1, color);
            let result = evaluate_board(&future, color, !color, 4, NEG_INF_SCORE, POS_INF_SCORE);
            // let val = heuristic_score(&future, color);

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
    alpha: u16,
    beta: u16,
) -> u16 {
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

fn heuristic_score(board: &Board, color: bool) -> u16 {
    let score = board.count_pieces(color);
    score
}

#[cfg(test)]
mod test {
    use crate::{
        board::Board,
        bots::deep_score_bot::{NEG_INF_SCORE, POS_INF_SCORE, evaluate_board},
        create_board,
    };

    #[test]
    pub fn evaluate_board_starting() {
        let board = create_board!(
            [_ _ _ _ _ _ _ _],
            [_ _ _ _ _ _ _ _],
            [_ _ _ _ _ _ _ _],
            [_ _ _ O X _ _ _],
            [_ _ _ X O X _ _],
            [_ _ _ _ _ O _ _],
            [_ _ _ _ _ _ _ _],
            [_ _ _ _ _ _ _ _],
        );

        assert_eq!(eval_board(&board, true, false, 0), 3);
    }

    #[test]
    pub fn eval_board_choices() {
        let board = create_board!(
            [_ _ _ _ _ _ _ _],
            [_ _ _ _ _ _ _ _],
            [_ X O _ _ _ _ _],
            [_ _ _ _ _ _ _ _],
            [_ _ _ _ _ _ _ _],
            [_ O X O O O _ _],
            [_ _ _ _ _ _ _ _],
            [_ _ _ _ _ _ _ _],
        );

        // at depth 0, we only see that we have two pieces on the board
        assert_eq!(eval_board(&board, false, false, 0), 2);

        // at depth 1, we see that we can place X on (6, 7) to get 6 points
        assert_eq!(eval_board(&board, false, false, 1), 6);

        // At depth 2, we see that our best move is to place on (4, 3) to get 4 points
        // The move at depth 1 is considered bad since O can take back immediately
        assert_eq!(eval_board(&board, false, false, 2), 4);

        // At depth 3, we see the sequence:
        //   - X (4, 3)
        //   - O skip
        //   - X (7, 6)
        assert_eq!(eval_board(&board, false, false, 3), 8);

        // At depth 4, we see the sequence:
        //   - X (1, 6)
        //   - O (1, 3)
        //   - X (7, 6)
        //   - O skip
        assert_eq!(eval_board(&board, false, false, 4), 7);

        // At depth 5, we see the best sequence:
        //   - X (3, 4)
        //   - O skip
        //   - X (1, 6)
        //   - O skip
        //   - X (7, 6)
        assert_eq!(eval_board(&board, false, false, 5), 10);
    }

    #[test]
    pub fn evaluate_board_game_opening_depth_4() {
        // these evaluations were the results of minmax before
        // implementing alpha-beta pruning

        // these evaluations were the results of minmax before
        // implementing alpha-beta pruning

        let board = create_board!(
            [_ _ _ _ _ _ _ _],
            [_ _ _ _ _ _ _ _],
            [_ _ _ _ _ _ _ _],
            [_ _ _ O X _ _ _],
            [_ _ _ O X X _ _],
            [_ _ _ O _ _ _ _],
            [_ _ _ _ _ _ _ _],
            [_ _ _ _ _ _ _ _],
        );

        assert_eq!(eval_board(&board, true, false, 4), 6);

        let board = create_board!(
            [_ _ _ _ _ _ _ _],
            [_ _ _ _ _ _ _ _],
            [_ _ _ _ _ _ _ _],
            [_ _ _ O O O _ _],
            [_ _ _ X O X _ _],
            [_ _ X O _ _ _ _],
            [_ _ _ _ _ _ _ _],
            [_ _ _ _ _ _ _ _],
        );

        assert_eq!(eval_board(&board, true, false, 4), 8);

        let board = create_board!(
            [_ _ _ _ _ _ _ _],
            [_ _ _ _ _ _ _ _],
            [_ _ _ X _ _ _ _],
            [_ _ _ X X O _ _],
            [_ _ _ X O O _ _],
            [_ _ X O _ O _ _],
            [_ _ _ _ _ _ _ _],
            [_ _ _ _ _ _ _ _],
        );

        assert_eq!(eval_board(&board, true, false, 4), 9);
    }

    fn eval_board(board: &Board, color: bool, next_turn_color: bool, depth: u16) -> u16 {
        evaluate_board(
            board,
            color,
            next_turn_color,
            depth,
            NEG_INF_SCORE,
            POS_INF_SCORE,
        )
    }
}
