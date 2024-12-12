use rand::{rngs::StdRng, SeedableRng};

use crate::{
    board::{Board, BOARD_SIZE},
    CellState, Color, Position,
};

use super::{add_noise, Evaluator};

pub fn positional_evaluate<B: Board>(board: &B, color: Color, epsilon: f32) -> i32 {
    let weights: [[i32; BOARD_SIZE]; BOARD_SIZE] = [
        [100, -20, 10, 5, 5, 10, -20, 100],
        [-20, -50, -2, -2, -2, -2, -50, -20],
        [10, -2, -1, -1, -1, -1, -2, 10],
        [5, -2, -1, -1, -1, -1, -2, 5],
        [5, -2, -1, -1, -1, -1, -2, 5],
        [10, -2, -1, -1, -1, -1, -2, 10],
        [-20, -50, -2, -2, -2, -2, -50, -20],
        [100, -20, 10, 5, 5, 10, -20, 100],
    ];

    let mut score = 0;

    (0..BOARD_SIZE).for_each(|y| {
        (0..BOARD_SIZE).for_each(|x| {
            let pos = Position::new(x, y);
            if let CellState::Disc(c) = board.get_cell_state(&pos) {
                if c == color {
                    score += weights[y][x];
                } else {
                    score -= weights[y][x];
                }
            }
        });
    });

    score
}

pub struct PositionalEvaluator {
    rng: StdRng,
    weights: [[i32; BOARD_SIZE]; BOARD_SIZE],
}

impl Default for PositionalEvaluator {
    fn default() -> Self {
        let weights: [[i32; BOARD_SIZE]; BOARD_SIZE] = [
            [100, -20, 10, 5, 5, 10, -20, 100],
            [-20, -50, -2, -2, -2, -2, -50, -20],
            [10, -2, -1, -1, -1, -1, -2, 10],
            [5, -2, -1, -1, -1, -1, -2, 5],
            [5, -2, -1, -1, -1, -1, -2, 5],
            [10, -2, -1, -1, -1, -1, -2, 10],
            [-20, -50, -2, -2, -2, -2, -50, -20],
            [100, -20, 10, 5, 5, 10, -20, 100],
        ];

        Self {
            rng: StdRng::from_entropy(),
            weights,
        }
    }
}

impl Evaluator for PositionalEvaluator {
    fn evaluate(&mut self, board: &crate::bit_board::BitBoard, color: Color, epsilon: f64) -> i32 {
        let mut score = 0;
        (0..BOARD_SIZE).for_each(|y| {
            (0..BOARD_SIZE).for_each(|x| {
                let pos = Position::new(x, y);
                if let CellState::Disc(c) = board.get_cell_state(&pos) {
                    if c == color {
                        score += self.weights[y][x];
                    } else {
                        score -= self.weights[y][x];
                    }
                }
            });
        });

        if epsilon == 0.0 {
            score
        } else {
            add_noise(score, epsilon, &mut self.rng)
        }
    }
}
