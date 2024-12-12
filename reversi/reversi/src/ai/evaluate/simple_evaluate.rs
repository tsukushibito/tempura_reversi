use rand::{rngs::StdRng, SeedableRng};

use crate::{board::Board, Color};

use super::{add_noise, Evaluator};

pub fn simple_evaluate<B: Board>(board: &B, color: Color) -> i32 {
    let black_count = board.black_count() as i32;
    let white_count = board.white_count() as i32;
    match color {
        Color::Black => black_count - white_count,
        Color::White => white_count - black_count,
    }
}

pub struct SimpleEvaluator {
    rng: StdRng,
}

impl Default for SimpleEvaluator {
    fn default() -> Self {
        Self {
            rng: StdRng::from_entropy(),
        }
    }
}

impl Evaluator for SimpleEvaluator {
    fn evaluate(&mut self, board: &crate::bit_board::BitBoard, color: Color, epsilon: f64) -> i32 {
        let black_count = board.black_count() as i32;
        let white_count = board.white_count() as i32;
        let value = match color {
            Color::Black => black_count - white_count,
            Color::White => white_count - black_count,
        };

        if epsilon == 0.0 {
            value
        } else {
            add_noise(value, epsilon, &mut self.rng)
        }
    }
}
