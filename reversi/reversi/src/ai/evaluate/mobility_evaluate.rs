use crate::{board::Board, Color};

use super::Evaluator;

pub struct MobilityEvaluator {}

impl Default for MobilityEvaluator {
    fn default() -> Self {
        Self {}
    }
}

impl Evaluator for MobilityEvaluator {
    fn evaluate(&mut self, board: &crate::bit_board::BitBoard, color: Color) -> i32 {
        let my_moves = board.get_valid_moves(color).len() as i32;
        let opponent_moves = board.get_valid_moves(color.opponent()).len() as i32;
        my_moves - opponent_moves
    }
}
