use crate::board::Board;

pub mod center_bot;
pub mod deep_minmax_bot;
pub mod deep_minmax_bot_pre_alpha_beta;
pub mod edge_bot;
pub mod edge_exclusive_bot;
pub mod first_valid_bot;
pub mod last_valid_bot;
pub mod minmax_score_bot;
pub mod random_bot;
pub mod shallow_score_bot;

pub struct BotRunner {
    pub board: Board,
    white_bot: Option<Box<dyn MakeMove>>,
    black_bot: Option<Box<dyn MakeMove>>,
}

impl BotRunner {
    pub fn new(
        black_bot: Option<Box<dyn MakeMove>>,
        white_bot: Option<Box<dyn MakeMove>>,
    ) -> BotRunner {
        BotRunner {
            black_bot,
            white_bot,
            board: Board::new(),
        }
    }

    pub fn run_game_to_end(&mut self) {
        let (black_bot, white_bot) = match (&mut self.black_bot, &mut self.white_bot) {
            (Some(black_bot), Some(white_bot)) => (black_bot, white_bot),
            _ => return,
        };

        loop {
            let mut moved = false;

            let black_move = black_bot.make_move(&self.board, false);
            if black_move.0 >= 0
                && black_move.1 >= 0
                && self.board.try_place_chip(black_move.0, black_move.1, false)
            {
                moved = true;
            }

            let white_move = white_bot.make_move(&self.board, true);
            if white_move.0 >= 0
                && white_move.1 >= 0
                && self.board.try_place_chip(white_move.0, white_move.1, true)
            {
                moved = true;
            }

            if !moved {
                break;
            }
        }
    }

    /// Runs the black bot's move once. Returns true if move was successful.
    /// If the move was unsuccessful, assume the opponent's victory.
    pub fn run_black_bot(&mut self) -> bool {
        if let Some(black_bot) = &mut self.black_bot {
            let black_move = black_bot.make_move(&self.board, false);
            return black_move.0 >= 0
                && black_move.1 >= 0
                && self.board.try_place_chip(black_move.0, black_move.1, false);
        } else {
            false
        }
    }

    /// Runs the white bot's move once. Returns true if move was successful.
    /// If the move was unsuccessful, assume the opponent's victory.
    pub fn run_white_bot(&mut self) -> bool {
        if let Some(white_bot) = &mut self.white_bot {
            let white_move = white_bot.make_move(&self.board, true);
            return white_move.0 >= 0
                && white_move.1 >= 0
                && self.board.try_place_chip(white_move.0, white_move.1, true);
        } else {
            false
        }
    }
}

pub trait MakeMove {
    fn make_move(&mut self, board: &Board, color: bool) -> (i8, i8);
}
