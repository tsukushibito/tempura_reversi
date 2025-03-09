use temp_reversi_core::{
    utils::{rotate_mask_180, rotate_mask_270_cw, rotate_mask_90_cw},
    Bitboard,
};

use super::pattern::Pattern;

/// Represents a group of patterns sharing the same state scores.
///
/// A `PatternGroup` contains multiple rotated `Pattern`s and a shared set of
/// state scores indexed by phase and state.
#[derive(Debug, Clone)]
pub struct PatternGroup {
    /// Rotated patterns belonging to this group.
    pub patterns: Vec<Pattern>,
    /// Shared state scores for all patterns in the group.
    /// Indexed as `state_scores[phase][state_index]`.
    pub state_scores: Vec<Vec<f32>>,
    /// Optional name for debugging or identification.
    pub name: Option<String>,

    old_board: Bitboard,
    old_score: f32,
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
    pub fn new(base_pattern: u64, state_scores: Vec<Vec<f32>>, name: Option<&str>) -> Self {
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
            old_board: Bitboard::new(0, 0),
            old_score: 0.0,
        }
    }

    /*
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
    */

    pub fn affected_patterns(&self, diff: u64) -> Vec<usize> {
        self.patterns
            .iter()
            .enumerate()
            .filter_map(|(i, pattern)| {
                if pattern.is_affected(diff) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn pattern_score(&self, board: &Bitboard, phase: usize, pattern_index: usize) -> f32 {
        if pattern_index >= self.patterns.len() {
            return 0.0;
        }
        let pattern = &self.patterns[pattern_index];
        let (black_mask, white_mask) = board.bits();
        let masked_black = black_mask & pattern.mask;
        let masked_white = white_mask & pattern.mask;
        if let Some(&state_index) = pattern.key_to_index.get(&(masked_black, masked_white)) {
            self.state_scores[phase][state_index]
        } else {
            0.0
        }
    }

    pub fn evaluate_score(&mut self, new_board: &Bitboard, phase: usize) -> f32 {
        let (old_black, old_white) = self.old_board.bits();
        let (new_black, new_white) = new_board.bits();
        let diff = (old_black ^ new_black) | (old_white ^ new_white);

        let affected_indices = self.affected_patterns(diff);

        let mut delta = 0.0;
        for idx in affected_indices {
            let old_pattern_score = self.pattern_score(&self.old_board, phase, idx);
            let new_pattern_score = self.pattern_score(new_board, phase, idx);
            delta += new_pattern_score - old_pattern_score;
        }

        let score = self.old_score + delta;

        self.old_score = score;
        self.old_board = new_board.clone();

        score
    }
}

#[cfg(test)]
mod tests {
    use temp_reversi_core::{
        utils::{rotate_mask_270_ccw, rotate_mask_90_ccw},
        Bitboard,
    };

    use super::*;

    /// Tests that each rotated pattern's `key_to_index` is consistent with the base pattern.
    ///
    /// The state index of each rotated pattern should match the index of the same board state
    /// in the base pattern after reversing the rotation.
    #[test]
    fn test_pattern_key_to_index_consistency() {
        let base_pattern: u64 = 0x0000000000070707; // Example pattern covering a 3x3 region
        let state_scores = vec![vec![0.0; 3_usize.pow(9)]]; // Dummy scores
        let pattern_group = PatternGroup::new(base_pattern, state_scores, Some("TestPattern"));

        let base = &pattern_group.patterns[0]; // Base (0-degree rotation) pattern

        for (i, pattern) in pattern_group.patterns.iter().enumerate() {
            if i == 0 {
                continue; // Skip the base pattern itself
            }

            for (&(black, white), &state_index) in &pattern.key_to_index {
                // Reverse the rotation to get the equivalent board state in the base pattern
                let (base_black, base_white) = match i {
                    1 => (rotate_mask_90_ccw(black), rotate_mask_90_ccw(white)), // 90-degree counterclockwise
                    2 => (rotate_mask_180(black), rotate_mask_180(white)),       // 180-degree
                    3 => (rotate_mask_270_ccw(black), rotate_mask_270_ccw(white)), // 270-degree counterclockwise
                    _ => (black, white),                                           // No rotation
                };

                // Ensure the state index matches the base pattern's key_to_index
                assert_eq!(
                    base.key_to_index.get(&(base_black, base_white)),
                    Some(&state_index),
                    "Mismatch in key_to_index for rotation {}",
                    i * 90
                );
            }
        }
    }

    /// Tests that `key_to_index` is correctly computed for a simple manually verifiable pattern.
    #[test]
    fn test_pattern_key_to_index_manual() {
        let simple_mask: u64 =
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01001100; // Example 3-bit pattern

        let pattern = Pattern::new(simple_mask, None);

        // Expected mapping of (black_mask, white_mask) to state indices
        let expected_key_to_index = vec![
            ((0b000000000, 0b000000000), 0), // Empty state
            ((0b000000100, 0b000000000), 1), // Black at (2,1)
            ((0b000000000, 0b000000100), 2), // White at (2,1)
            ((0b000001000, 0b000000000), 3), // Black at (2,2)
            ((0b000001100, 0b000000000), 4), // Black at (2,1) and (2,2)
            ((0b000001000, 0b000000100), 5), // Black at (2,2), White at (2,1)
            ((0b000000000, 0b000001000), 6), // White at (2,2)
            ((0b000000100, 0b000001000), 7), // Black at (2,1), White at (2,2)
            ((0b000000000, 0b000001100), 8), // White at (2,1) and (2,2)
        ];

        for &(key, expected_index) in &expected_key_to_index {
            assert_eq!(
                pattern.key_to_index.get(&key),
                Some(&expected_index),
                "Incorrect state index for key {:?}",
                key
            );
        }
    }

    /// Tests that `evaluate_score()` produces consistent results across rotated versions of the same board.
    #[test]
    fn test_pattern_group_evaluate_score() {
        let base_pattern: u64 = 0x0000000000070707; // Example pattern covering a 3x3 region

        // Create state scores where the score is equal to the state index for easy verification
        let mut state_scores = vec![vec![10.0; 3_usize.pow(9)]];
        state_scores[0]
            .iter_mut()
            .enumerate()
            .for_each(|(i, score)| {
                *score = i as f32; // Assign state index as score
            });

        let mut pattern_group = PatternGroup::new(base_pattern, state_scores, Some("TestPattern"));

        // Example board with black pieces in the lower part and white in the upper
        let original_board = Bitboard::new(0x0000000000070000, 0x0000000000000700);

        // Rotate the board in different orientations
        let rotated_90_board = Bitboard::new(
            rotate_mask_90_cw(original_board.bits().0),
            rotate_mask_90_cw(original_board.bits().1),
        );
        let rotated_180_board = Bitboard::new(
            rotate_mask_180(original_board.bits().0),
            rotate_mask_180(original_board.bits().1),
        );
        let rotated_270_board = Bitboard::new(
            rotate_mask_270_cw(original_board.bits().0),
            rotate_mask_270_cw(original_board.bits().1),
        );

        // Compute scores for each rotated board
        let original_score = pattern_group.evaluate_score(&original_board, 0);
        let rotated_90_score = pattern_group.evaluate_score(&rotated_90_board, 0);
        let rotated_180_score = pattern_group.evaluate_score(&rotated_180_board, 0);
        let rotated_270_score = pattern_group.evaluate_score(&rotated_270_board, 0);

        // Ensure scores are consistent across all rotations
        assert_eq!(
            original_score, rotated_90_score,
            "Score mismatch for 90-degree rotation"
        );
        assert_eq!(
            original_score, rotated_180_score,
            "Score mismatch for 180-degree rotation"
        );
        assert_eq!(
            original_score, rotated_270_score,
            "Score mismatch for 270-degree rotation"
        );
    }
}
