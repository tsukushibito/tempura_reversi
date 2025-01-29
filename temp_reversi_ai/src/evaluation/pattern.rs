use super::EvaluationFunction;
use crate::patterns::PatternGroup;
use temp_reversi_core::{Bitboard, Player};

/// Evaluates the board based on multiple pattern groups and their scores.
pub struct PatternEvaluator {
    /// Collection of pattern groups.
    pub groups: Vec<PatternGroup>,
}

impl PatternEvaluator {
    /// Creates a `PatternEvaluator` with a predefined list of pattern groups.
    ///
    /// # Arguments
    /// * `groups` - A vector of `PatternGroup` instances to be managed by the evaluator.
    ///
    /// # Returns
    /// A `PatternEvaluator` initialized with the provided pattern groups.
    pub fn new(groups: Vec<PatternGroup>) -> Self {
        Self { groups }
    }
}

impl EvaluationFunction for PatternEvaluator {
    fn evaluate(&self, board: &Bitboard, player: Player) -> i32 {
        let mut total_score = 0;

        // Calculate the phase using `Bitboard::count_stones`
        let (black_stones, white_stones) = board.count_stones();
        let total_stones = black_stones + white_stones;
        let phase = 60 - total_stones.min(60); // Phase is capped at 59

        // Iterate through all pattern groups and accumulate scores
        for group in &self.groups {
            total_score += group.evaluate_score(board, phase);
        }

        // Adjust score based on the perspective of the current player
        if player == Player::White {
            total_score = -total_score;
        }

        total_score
    }
}
