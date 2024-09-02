use crate::{board::Board, Color};

use super::Evaluator;

#[derive(Debug, Default)]
pub struct SimpleEvaluator {}

impl Evaluator for SimpleEvaluator {
    fn evaluate(&self, board: &crate::bit_board::BitBoard, color: Color) -> i32 {
        let black_count = board.black_count() as i32;
        let white_count = board.white_count() as i32;
        match color {
            Color::Black => black_count - white_count,
            Color::White => white_count - black_count,
        }
    }
}
