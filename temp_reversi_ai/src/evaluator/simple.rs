use temp_reversi_core::{Board, Player};

use super::EvaluationFunction;

#[derive(Clone)]
pub struct SimpleEvaluator;

impl<B: Board> EvaluationFunction<B> for SimpleEvaluator {
    fn evaluate(&self, board: &B, player: Player) -> i32 {
        let (black_count, white_count) = board.count_stones();
        match player {
            Player::Black => black_count as i32 - white_count as i32,
            Player::White => white_count as i32 - black_count as i32,
        }
    }
}
