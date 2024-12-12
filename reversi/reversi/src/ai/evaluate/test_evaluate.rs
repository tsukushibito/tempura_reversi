use rand::{rngs::StdRng, SeedableRng};

use crate::{board::Board, CellState, Color};

use super::{
    add_noise,
    mobility_evaluate::{mobility_evaluate, MobilityEvaluator},
    simple_evaluate::{simple_evaluate, SimpleEvaluator},
    Evaluator,
};

pub fn test_evaluate<B: Board>(board: &B, color: Color) -> i32 {
    let empty_count = board.count_of(CellState::Empty);
    if empty_count > 10 {
        mobility_evaluate(board, color, 0.0)
    } else {
        simple_evaluate(board, color)
    }
}

pub struct TestEvaluator {
    rng: StdRng,
    mobility: MobilityEvaluator,
    simple: SimpleEvaluator,
}

impl Default for TestEvaluator {
    fn default() -> Self {
        Self {
            rng: StdRng::from_entropy(),
            mobility: Default::default(),
            simple: Default::default(),
        }
    }
}

impl Evaluator for TestEvaluator {
    fn evaluate(&mut self, board: &crate::bit_board::BitBoard, color: Color, epsilon: f64) -> i32 {
        let empty_count = board.count_of(CellState::Empty);
        let value = if empty_count > 10 {
            self.mobility.evaluate(board, color, epsilon)
        } else {
            self.simple.evaluate(board, color, epsilon)
        };

        if epsilon == 0.0 {
            value
        } else {
            add_noise(value, epsilon, &mut self.rng)
        }
    }
}
