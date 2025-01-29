use temp_reversi_core::{
    utils::{rotate_mask_180, rotate_mask_270_cw, rotate_mask_90_cw},
    Bitboard,
};

use super::pattern::Pattern;

/// Represents a group of patterns sharing the same state scores.
///
/// A `PatternGroup` contains multiple rotated `Pattern`s and a shared set of
/// state scores indexed by phase and state.
pub struct PatternGroup {
    /// Rotated patterns belonging to this group.
    pub patterns: Vec<Pattern>,
    /// Shared state scores for all patterns in the group.
    /// Indexed as `state_scores[phase][state_index]`.
    pub state_scores: Vec<Vec<i32>>,
    /// Optional name for debugging or identification.
    pub name: Option<String>,
}

impl PatternGroup {
    /// Creates a new `PatternGroup` with precomputed patterns and shared state scores.
    ///
    /// # Arguments
    /// * `base_pattern` - A 64-bit integer representing the base bitmask of the pattern.
    /// * `state_scores` - A 2D vector containing scores indexed by phase and state.
    /// * `name` - An optional name for the pattern group.
    ///
    /// # Returns
    /// A `PatternGroup` struct containing the rotated patterns and shared state scores.
    pub fn new(base_pattern: u64, state_scores: Vec<Vec<i32>>, name: Option<&str>) -> Self {
        let base_pattern_obj = Pattern::new(base_pattern, None);

        let rotated_90 = Pattern::new(
            rotate_mask_90_cw(base_pattern),
            Some((&base_pattern_obj, 1)),
        );
        let rotated_180 = Pattern::new(rotate_mask_180(base_pattern), Some((&base_pattern_obj, 2)));
        let rotated_270 = Pattern::new(
            rotate_mask_270_cw(base_pattern),
            Some((&base_pattern_obj, 3)),
        );

        Self {
            patterns: vec![base_pattern_obj, rotated_90, rotated_180, rotated_270],
            state_scores,
            name: name.map(|s| s.to_string()),
        }
    }

    /// Evaluates the score contribution of this pattern group for the given board state.
    ///
    /// # Arguments
    /// * `board` - The current board state as a `Bitboard`.
    /// * `phase` - Current game phase (0-59).
    ///
    /// # Returns
    /// * `i32` - The score contribution of this pattern group.
    pub fn evaluate_score(&self, board: &Bitboard, phase: usize) -> i32 {
        let mut score = 0;
        let (black_mask, white_mask) = board.bits(); // Get black and white bit masks

        for pattern in &self.patterns {
            let masked_black = black_mask & pattern.mask;
            let masked_white = white_mask & pattern.mask;

            if let Some(&state_index) = pattern.key_to_index.get(&(masked_black, masked_white)) {
                score += self.state_scores[phase][state_index];
            }
        }

        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_group_creation() {
        let base_pattern = 0b000000000_000000111_000000000_000000000_000000000; // Horizontal line
        let state_scores = vec![
            vec![1; 3_usize.pow(3)], // Phase 0 scores
            vec![2; 3_usize.pow(3)], // Phase 1 scores
        ];

        let group = PatternGroup::new(base_pattern, state_scores.clone(), Some("Test Group"));

        // Verify the number of patterns
        assert_eq!(group.patterns.len(), 4);

        // Verify the shared state scores
        assert_eq!(group.state_scores.len(), state_scores.len());
        assert_eq!(group.state_scores[0].len(), state_scores[0].len());

        // Verify the group name
        assert_eq!(group.name.as_deref(), Some("Test Group"));
    }
}
