use rand::{rngs::StdRng, SeedableRng};

use crate::{board::Board, CellState, Color};

use super::{mobility_evaluate::MobilityEvaluator, simple_evaluate::SimpleEvaluator, Evaluator};

pub struct TestEvaluator {
    mobility: MobilityEvaluator,
    simple: SimpleEvaluator,
}

impl Default for TestEvaluator {
    fn default() -> Self {
        Self {
            mobility: Default::default(),
            simple: Default::default(),
        }
    }
}

impl Evaluator for TestEvaluator {
    fn evaluate(&mut self, board: &crate::bit_board::BitBoard, color: Color) -> i32 {
        let empty_count = board.count_of(CellState::Empty);
        if empty_count > 10 {
            self.mobility.evaluate(board, color)
        } else {
            self.simple.evaluate(board, color)
        }
    }
}
