use crate::{board::Board, CellState, Color};

use super::{
    mobility_evaluator::MobilityEvaluator, simple_evaluator::SimpleEvaluator, Evaluator,
    PositionalEvaluator,
};

#[derive(Debug, Default)]
pub struct TestEvaluator {
    mobility: MobilityEvaluator,
    positional: PositionalEvaluator,
    simple: SimpleEvaluator,
}

impl Evaluator for TestEvaluator {
    fn evaluate(&self, board: &crate::bit_board::BitBoard, color: Color) -> i32 {
        let empty_count = board.count_of(CellState::Empty);
        if empty_count > 10 {
            let positional = self.positional.evaluate(board, color) as f32;
            let mobility = self.mobility.evaluate(board, color);
            mobility + (positional * 0.25f32) as i32
        } else {
            self.simple.evaluate(board, color)
        }
    }
}
