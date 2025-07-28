mod board;
mod bots;
mod js_console;
mod macros;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    board::Board,
    bots::{BotRunner, MakeMove},
};

#[wasm_bindgen]
struct JsInterface {
    bot_runner: Option<BotRunner>,
    white_bot: Option<Box<dyn MakeMove>>,
    black_bot: Option<Box<dyn MakeMove>>,
    staged_bot: Option<Box<dyn MakeMove>>,
}

#[wasm_bindgen]
#[allow(dead_code)]
impl JsInterface {
    pub fn new() -> JsInterface {
        JsInterface {
            bot_runner: None,
            white_bot: None,
            black_bot: None,
            staged_bot: None,
        }
    }

    pub fn create_new_center_bot(&mut self) {
        self.staged_bot = Some(Box::new(bots::center_bot::Bot::new()));
    }

    pub fn create_deep_heuristic_bot(&mut self) {
        self.staged_bot = Some(Box::new(bots::deep_heuristic_bot::Bot::new()));
    }

    pub fn create_deep_negative_heuristic_bot(&mut self) {
        self.staged_bot = Some(Box::new(bots::deep_negative_heuristic_bot::Bot::new()));
    }

    pub fn create_deep_score_bot(&mut self) {
        self.staged_bot = Some(Box::new(bots::deep_score_bot::Bot::new()));
    }

    pub fn create_new_edge_bot(&mut self) {
        self.staged_bot = Some(Box::new(bots::edge_bot::Bot::new()));
    }

    pub fn create_new_edge_exclusive_bot(&mut self) {
        self.staged_bot = Some(Box::new(bots::edge_exclusive_bot::Bot::new()));
    }

    pub fn create_new_first_valid_move_bot(&mut self) {
        self.staged_bot = Some(Box::new(bots::first_valid_bot::Bot::new()));
    }

    pub fn create_new_last_valid_move_bot(&mut self) {
        self.staged_bot = Some(Box::new(bots::last_valid_bot::Bot::new()))
    }

    pub fn create_new_minmax_score_bot(&mut self) {
        self.staged_bot = Some(Box::new(bots::minmax_score_bot::Bot::new()));
    }

    pub fn create_new_random_bot(&mut self) {
        self.staged_bot = Some(Box::new(bots::random_bot::Bot::new()))
    }

    pub fn create_shallow_score_bot(&mut self) {
        self.staged_bot = Some(Box::new(bots::shallow_score_bot::Bot::new()))
    }

    pub fn set_bot_as_white(&mut self) {
        self.white_bot = self.staged_bot.take();
    }

    pub fn set_bot_as_black(&mut self) {
        self.black_bot = self.staged_bot.take();
    }

    pub fn create_game(&mut self) {
        self.bot_runner = Some(BotRunner::new(self.black_bot.take(), self.white_bot.take()));
    }

    pub fn get_board_filled(&self) -> u64 {
        let runner = self.get_runner();
        runner.board.filled
    }

    pub fn get_board_color(&self) -> u64 {
        let runner = self.get_runner();
        runner.board.color
    }

    pub fn board_try_place(&mut self, x: i8, y: i8, color: bool) -> bool {
        let runner = self.get_runner_mut();
        runner.board.try_place_chip(x, y, color)
    }

    /// Checks if there is a valid move for a player
    pub fn board_has_valid_move(&self, color: bool) -> bool {
        let runner = self.get_runner();
        runner.board.has_valid_move(color)
    }

    /// Counts the number of pieces of a color
    pub fn board_count_pieces(&self, color: bool) -> u16 {
        let runner = self.get_runner();
        runner.board.count_pieces(color)
    }

    pub fn bot_run_to_end(&mut self) {
        let runner = self.get_runner_mut();
        runner.run_game_to_end();
    }

    pub fn bot_run_to_end_times(&mut self, times: u32) -> Vec<u32> {
        let runner = self.get_runner_mut();

        let mut white_wins = 0;
        let mut black_wins = 0;

        for _ in 0..times {
            runner.board = Board::new();
            runner.run_game_to_end();

            // note: this is inaccurate -- run game to end should check if the bot forfeited.
            // if a bot forfeits with more score, then we shouldn't count it a win for the bot.
            let whites = runner.board.count_pieces(true);
            let blacks = runner.board.count_pieces(false);
            if whites > blacks {
                white_wins += 1;
            } else if whites < blacks {
                black_wins += 1;
            }
        }

        vec![black_wins, white_wins]
    }

    pub fn bot_run_white(&mut self) -> Option<Vec<i8>> {
        let runner = self.get_runner_mut();
        let m = runner.run_white_bot()?;
        Some(vec![m.0, m.1])
    }

    pub fn bot_run_black(&mut self) -> Option<Vec<i8>> {
        let runner = self.get_runner_mut();
        let m = runner.run_black_bot()?;
        Some(vec![m.0, m.1])
    }

    fn get_runner(&self) -> &BotRunner {
        self.bot_runner
            .as_ref()
            .expect("Bot should be initialized first")
    }

    fn get_runner_mut(&mut self) -> &mut BotRunner {
        self.bot_runner
            .as_mut()
            .expect("Bot should be initalized first")
    }
}
