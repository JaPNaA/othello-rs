use crate::{board::Board, bots::MakeMove, js_console};

/// This bot maximizes score by doing a 5-deep minmax search
/// note: currently doesn't seem to work(?)
pub struct Bot {}

impl Bot {
    pub fn new() -> Bot {
        Bot {}
    }
}

impl MakeMove for Bot {
    fn make_move(&mut self, board: &Board, color: bool) -> (i8, i8) {
        let moves = board.get_all_valid_moves(color);

        match moves.into_iter().max_by_key(|m| {
            let mut future = board.clone();
            future.try_place_chip(m.0, m.1, color);
            let val = evaluate_board(&future, color, !color, 4);
            // let val = heuristic_score(&future, color);
            js_console::log(&format!("Score: {} for move {}, {}", val, m.0, m.1));
            val
        }) {
            Some(x) => x,
            None => (-1, -1),
        }
    }
}

fn evaluate_board(board: &Board, color: bool, next_turn_color: bool, depth: u16) -> u16 {
    if depth == 0 {
        return heuristic_score(board, color);
    }

    let next_moves = board
        .get_all_valid_moves(next_turn_color)
        .into_iter()
        .map(|m| {
            let mut future_board = board.clone();
            future_board.try_place_chip(m.0, m.1, next_turn_color);
            evaluate_board(&future_board, color, !next_turn_color, depth - 1)
        });

    if next_turn_color == color {
        // our move, find max
        next_moves.max()
    } else {
        // their move, find min
        next_moves.min()
    }
    .unwrap_or_else(|| evaluate_board(&board, color, !next_turn_color, depth - 1))
}

fn heuristic_score(board: &Board, color: bool) -> u16 {
    board.count_pieces(color)
}

#[cfg(test)]
mod test {
    use crate::{board::Board, bots::deep_minmax_bot::evaluate_board};

    macro_rules! create_board_piece {
        (X) => {
            1
        };
        (O) => {
            2
        };
        (_) => {
            3
        };
    }

    macro_rules! create_board_row {
        ([$a:tt $b:tt $c:tt $d:tt $e:tt $f:tt $g:tt $h:tt]) => {
            [
                create_board_piece!($a),
                create_board_piece!($b),
                create_board_piece!($c),
                create_board_piece!($d),
                create_board_piece!($e),
                create_board_piece!($f),
                create_board_piece!($g),
                create_board_piece!($h),
            ]
        };
    }

    macro_rules! create_board {
        ($a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt, $h:tt,) => {{
            let mut board = Board::new();
            let data = [
                create_board_row!($a),
                create_board_row!($b),
                create_board_row!($c),
                create_board_row!($d),
                create_board_row!($e),
                create_board_row!($f),
                create_board_row!($g),
                create_board_row!($h),
            ];

            for y in 0..8usize {
                for x in 0..8usize {
                    match data[y][x] {
                        1 => board.set(x as i8, y as i8, false),
                        2 => board.set(x as i8, y as i8, true),
                        3 => board.clear(x as i8, y as i8),
                        _ => { }
                    }
                }
            }

            board
        }};
    }

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

        assert_eq!(evaluate_board(&board, true, false, 0), 3);
    }

    #[test]
    pub fn evaluate_board_choices() {
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
        assert_eq!(evaluate_board(&board, false, false, 0), 2);

        // at depth 1, we see that we can place X on (6, 7) to get 6 points
        assert_eq!(evaluate_board(&board, false, false, 1), 6);

        // At depth 2, we see that our best move is to place on (4, 3) to get 4 points
        // The move at depth 1 is considered bad since O can take back immediately
        assert_eq!(evaluate_board(&board, false, false, 2), 4);

        // At depth 3, we see the sequence:
        //   - X (4, 3)
        //   - O skip
        //   - X (7, 6)
        assert_eq!(evaluate_board(&board, false, false, 3), 8);

        // At depth 4, we see the sequence:
        //   - X (1, 6)
        //   - O (1, 3)
        //   - X (7, 6)
        //   - O skip
        assert_eq!(evaluate_board(&board, false, false, 4), 7);

        // At depth 5, we see the best sequence:
        //   - X (3, 4)
        //   - O skip
        //   - X (1, 6)
        //   - O skip
        //   - X (7, 6)
        assert_eq!(evaluate_board(&board, false, false, 5), 10);
    }
}
