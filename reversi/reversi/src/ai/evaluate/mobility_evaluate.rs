use rand::{rngs::StdRng, SeedableRng};
use rand_distr::{Distribution, Normal};

use crate::{board::Board, Color};

use super::{add_noise, Evaluator};

pub fn mobility_evaluate<B: Board>(board: &B, color: Color, epsilon: f32) -> i32 {
    let my_moves = board.get_valid_moves(color).len() as i32;
    let opponent_moves = board.get_valid_moves(color.opponent()).len() as i32;
    ((my_moves - opponent_moves) as f32 * (1f32 - epsilon)) as i32
}

pub struct MobilityEvaluator {
    rng: StdRng,
}

impl Default for MobilityEvaluator {
    fn default() -> Self {
        Self {
            rng: StdRng::from_entropy(),
        }
    }
}

impl Evaluator for MobilityEvaluator {
    fn evaluate(&mut self, board: &crate::bit_board::BitBoard, color: Color, epsilon: f64) -> i32 {
        let my_moves = board.get_valid_moves(color).len() as i32;
        let opponent_moves = board.get_valid_moves(color.opponent()).len() as i32;
        let value = my_moves - opponent_moves;

        if epsilon == 0.0 {
            value
        } else {
            add_noise(value, epsilon, &mut self.rng)
        }
    }
}
