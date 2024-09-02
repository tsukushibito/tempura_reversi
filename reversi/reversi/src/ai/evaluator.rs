mod mobility_evaluator;
mod positional_evaluator;
mod simple_evaluator;
mod tempura_evaluator;
mod test_evaluator;

pub use mobility_evaluator::MobilityEvaluator;
pub use positional_evaluator::PositionalEvaluator;
pub use simple_evaluator::SimpleEvaluator;
pub use tempura_evaluator::TempuraEvaluator;
pub use test_evaluator::TestEvaluator;

use crate::{bit_board::BitBoard, Color};

pub trait Evaluator {
    fn evaluate(&self, board: &BitBoard, color: Color) -> i32;
}

pub fn add_noise(value: i32, epsilon: f64, rng: &mut impl rand::Rng) -> i32 {
    use rand_distr::Distribution;
    let normal = rand_distr::Normal::new(0.0, epsilon).unwrap();
    let noise = normal.sample(rng);
    value + noise as i32
}
