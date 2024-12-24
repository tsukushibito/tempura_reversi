mod mobility_evaluate;
mod model;
mod pattern;
mod pattern_evaluate;
mod positional_evaluate;
mod simple_evaluate;
mod test_evaluate;

pub use mobility_evaluate::MobilityEvaluator;
pub use model::*;
pub use pattern::*;
pub use pattern_evaluate::*;
pub use positional_evaluate::PositionalEvaluator;
pub use simple_evaluate::SimpleEvaluator;
pub use test_evaluate::TestEvaluator;

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
