use std::collections::HashMap;

use temp_reversi_core::Bitboard;

use crate::{evaluation::PatternEvaluator, patterns::get_predefined_patterns, utils::SparseVector};

/// Extracts a feature vector from the board state using predefined pattern groups.
///
/// The extracted feature vector uses a **sparse representation**, where each feature corresponds
/// to a specific pattern state index. The values range from `0` to `4`, indicating how many
/// rotated versions of the pattern match the current board state.
///
/// # Feature Representation
/// - Each `PatternGroup` has a unique index offset.
/// - All rotated versions of a pattern contribute to the same feature.
/// - The final feature values range from `0` to `4`, indicating the number of matching rotated patterns.
///
/// # Arguments
/// * `board` - The current board state as a `Bitboard`.
///
/// # Returns
/// * A `SparseVector` representing the board's features.
///
/// # Example
/// ```
/// let board = Bitboard::default();
/// let features = extract_features(&board);
/// assert!(!features.indices().is_empty());
/// ```
pub fn extract_features(board: &Bitboard) -> SparseVector {
    let evaluator = PatternEvaluator::new(get_predefined_patterns());
    let (black_mask, white_mask) = board.bits();

    // Store feature counts (each key is a unique feature index, value is the occurrence count)
    let mut feature_counts: HashMap<usize, f32> = HashMap::new();

    // Keeps track of the offset for each PatternGroup to ensure unique feature indices
    let mut feature_index_offset = 0;
    let mut total_features = 0;

    for group in &evaluator.groups {
        // The total number of possible states for this pattern group (3^N)
        let num_states_per_group = group.state_scores[0].len();

        for pattern in &group.patterns {
            let masked_black = black_mask & pattern.mask;
            let masked_white = white_mask & pattern.mask;

            if let Some(&state_index) = pattern.key_to_index.get(&(masked_black, masked_white)) {
                let feature_index = feature_index_offset + state_index;
                *feature_counts.entry(feature_index).or_insert(0.0) += 1.0;
            }
        }

        // Move the offset forward by the total number of states in this PatternGroup
        feature_index_offset += num_states_per_group;
        total_features += num_states_per_group;
    }

    // Convert feature counts into a SparseVector representation
    let (indices, values): (Vec<usize>, Vec<f32>) = feature_counts.into_iter().unzip();
    SparseVector::new(indices, values, total_features).expect("Failed to create SparseVector")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests feature extraction on the default board state.
    ///
    /// Ensures that the extracted sparse vector:
    /// - Is not empty (at least some features should be present).
    /// - Has values in the expected range (0 to 4).
    #[test]
    fn test_extract_features_default_board() {
        let board = Bitboard::default();
        let features = extract_features(&board);

        // Ensure that some features are extracted (the vector is not empty)
        assert!(
            !features.indices().is_empty(),
            "Feature vector should not be empty for the default board."
        );

        // Ensure all values are within the valid range [0, 4]
        for &value in features.values() {
            assert!(
                (0.0..=4.0).contains(&value),
                "Feature values should be in range 0 to 4, but found {}",
                value
            );
        }
    }
}
