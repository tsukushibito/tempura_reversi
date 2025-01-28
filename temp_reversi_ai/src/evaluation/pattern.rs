use super::EvaluationFunction;
use crate::patterns::PatternGroup;
use temp_reversi_core::Player;

/// Evaluates the board based on multiple pattern groups and their scores.
pub struct PatternEvaluator {
    /// Collection of pattern groups.
    pub groups: Vec<PatternGroup>,
}

impl PatternEvaluator {
    /// Creates a new `PatternEvaluator`.
    pub fn new() -> Self {
        Self { groups: Vec::new() }
    }

    /// Adds a new `PatternGroup` to the manager.
    ///
    /// # Arguments
    /// * `group` - The `PatternGroup` to add.
    pub fn add_group(&mut self, group: PatternGroup) {
        self.groups.push(group);
    }
}

impl EvaluationFunction for PatternEvaluator {
    fn evaluate(
        &self,
        board: &temp_reversi_core::Bitboard,
        player: temp_reversi_core::Player,
    ) -> i32 {
        let mut total_score = 0;

        // Get black and white masks from the board
        let (black_mask, white_mask) = board.bits();

        // Calculate the number of stones to determine the phase (0 to 59)
        let total_stones = (black_mask | white_mask).count_ones() as usize;
        let phase = 60 - total_stones.min(60); // Phase is capped at 59

        // Iterate through all pattern groups and accumulate scores
        for group in &self.groups {
            for pattern in &group.patterns {
                let masked_black = black_mask & pattern.mask;
                let masked_white = white_mask & pattern.mask;

                if let Some(&state_index) = pattern.key_to_index.get(&(masked_black, masked_white))
                {
                    total_score += group.state_scores[phase][state_index];
                }
            }
        }

        // Adjust score based on the perspective of the current player
        if player == Player::White {
            total_score = -total_score;
        }

        total_score
    }
}
