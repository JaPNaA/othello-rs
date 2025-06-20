use crate::{board::Board, bots::MakeMove};

/// This bot always chooses the first valid move.
pub struct Bot {}

impl Bot {
    pub fn new() -> Bot {
        Bot {}
    }
}

/// This bot always chooses the first valid move
impl MakeMove for Bot {
    fn make_move(&mut self, board: &Board, color: bool) -> (i8, i8) {
        for y in 0..8 {
            for x in 0..8 {
                if board.is_valid_move(x, y, color) {
                    return (x, y);
                }
            }
        }

        (-1, -1)
    }
}
