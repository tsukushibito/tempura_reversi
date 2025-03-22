use temp_game_ai::Evaluator;
use temp_reversi_core::Player;

use crate::ReversiState;

#[derive(Clone)]
pub struct SimpleEvaluator;

impl Evaluator<ReversiState> for SimpleEvaluator {
    fn evaluate(&mut self, state: &ReversiState) -> i32 {
        let (black_count, white_count) = state.board.count_stones();
        match state.player {
            Player::Black => black_count as i32 - white_count as i32,
            Player::White => white_count as i32 - black_count as i32,
        }
    }
}
