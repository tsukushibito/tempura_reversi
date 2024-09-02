use temp_reversi_core::{Bitboard, Player};

use super::EvaluationFunction;

pub struct SimpleEvaluator;

impl EvaluationFunction for SimpleEvaluator {
    fn evaluate(&self, board: &Bitboard, player: Player) -> i32 {
        let (black_count, white_count) = board.count_stones();
        match player {
            Player::Black => black_count as i32 - white_count as i32,
            Player::White => white_count as i32 - black_count as i32,
        }
    }
}