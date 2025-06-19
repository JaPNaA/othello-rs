use crate::{board::Board, bots::MakeMove};

pub struct Bot {}

impl Bot {
    pub fn new() -> Bot {
        Bot {}
    }
}

impl MakeMove for Bot {
    fn make_move(&self, board: &Board, color: bool) -> (i8, i8) {
        for y in 0..8 {
            for x in 0..8 {
                if board.is_valid_move(x, y, color) {
                    return (x, y);
                }
            }
        }

        (0, 0)
    }
}
