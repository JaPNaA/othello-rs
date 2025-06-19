use crate::board::Board;

pub mod first_valid_bot;
pub mod random_bot;

pub struct BotRunner {
    pub board: Board,
    white_bot: Box<dyn MakeMove>,
    black_bot: Box<dyn MakeMove>,
}

impl BotRunner {
    pub fn new(black_bot: Box<dyn MakeMove>, white_bot: Box<dyn MakeMove>) -> BotRunner {
        BotRunner {
            black_bot,
            white_bot,
            board: Board::new(),
        }
    }

    pub fn run_game_to_end(&mut self) {
        loop {
            let black_move = self.black_bot.make_move(&self.board, false);
            if black_move.0 < 0
                || black_move.1 < 0
                || !self.board.try_place_chip(black_move.0, black_move.1, false)
            {
                break;
            }

            let white_move = self.white_bot.make_move(&self.board, true);
            if white_move.0 < 0
                || white_move.1 < 0
                || !self.board.try_place_chip(white_move.0, white_move.1, true)
            {
                break;
            }
        }
    }
}

pub trait MakeMove {
    fn make_move(&self, board: &Board, color: bool) -> (i8, i8);
}
