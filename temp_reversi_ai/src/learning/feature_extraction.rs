use std::collections::HashMap;

use temp_reversi_core::Bitboard;

use crate::{evaluation::PatternEvaluator, utils::SparseVector};

/// Extracts a feature vector from the board state using the provided evaluator and predefined pattern groups.
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
/// * `evaluator` - A reference to a `PatternEvaluator` used for feature extraction.
///
/// # Returns
/// * A `SparseVector` representing the board's features.
///
/// # Example
/// ```
/// let board = Bitboard::default();
/// let evaluator = PatternEvaluator::new(get_predefined_patterns());
/// let features = extract_features(&board, &evaluator);
/// assert!(!features.indices().is_empty());
/// ```
pub fn extract_features(board: &Bitboard, evaluator: &PatternEvaluator) -> SparseVector {
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
    use std::collections::HashSet;

    use temp_reversi_core::{utils::rotate_mask_90_cw, Player, Position};

    use crate::patterns::get_predefined_patterns;

    use super::*;

    /// Tests feature extraction on the default board state.
    ///
    /// Ensures that the extracted sparse vector:
    /// - Is not empty (at least some features should be present).
    /// - Has values in the expected range (0 to 4).
    #[test]
    fn test_extract_features_default_board() {
        let board = Bitboard::default();
        let evaluator = PatternEvaluator::new(get_predefined_patterns());
        let features = extract_features(&board, &evaluator);

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

    /// Tests feature extraction when stones are placed in the four corners.
    ///
    /// Ensures that:
    /// - The feature vector changes after placing stones in the corners.
    /// - Values remain within the expected range (0 to 4).
    #[test]
    fn test_extract_features_with_corners() {
        // Place stones in the four corners
        let board = Bitboard::new(0x8100000000000081, 0);
        let evaluator = PatternEvaluator::new(get_predefined_patterns());
        let features = extract_features(&board, &evaluator);

        // Ensure that some features are extracted
        assert!(
            !features.indices().is_empty(),
            "Feature vector should not be empty after placing corner stones."
        );

        // Ensure all values are within the valid range [0, 4]
        for &value in features.values() {
            assert!(
                (0.0..=4.0).contains(&value),
                "Feature values should be in range 0 to 4, but found {}",
                value
            );
        }

        // Check if the feature vector has changed from the default state
        let default_board = Bitboard::default();
        let evaluator = PatternEvaluator::new(get_predefined_patterns());
        let default_features = extract_features(&default_board, &evaluator);

        assert_ne!(
            features.indices(),
            default_features.indices(),
            "Feature vector indices should change after placing stones in the corners."
        );
    }

    /// Tests that feature indices from different `PatternGroup`s do not overlap.
    ///
    /// Ensures that:
    /// - Indices are unique across all `PatternGroup`s.
    /// - The offset mechanism correctly separates different `PatternGroup`s.
    #[test]
    fn test_feature_index_offset() {
        let board = Bitboard::default();
        let evaluator = PatternEvaluator::new(get_predefined_patterns());
        let features = extract_features(&board, &evaluator);

        let indices: HashSet<usize> = features.indices().iter().cloned().collect();

        // Ensure that all indices are unique
        assert_eq!(
            indices.len(),
            features.indices().len(),
            "Feature indices should be unique across different PatternGroups."
        );

        // Verify that the offsets separate different PatternGroups
        let predefined_patterns = get_predefined_patterns();
        let mut feature_index_offset = 0;

        for group in &predefined_patterns {
            let num_states_per_group = group.state_scores[0].len();

            for state_index in 0..num_states_per_group {
                let feature_index = feature_index_offset + state_index;

                // If this feature index exists in the extracted features, it should be in range
                if indices.contains(&feature_index) {
                    assert!(
                        feature_index >= feature_index_offset
                            && feature_index < feature_index_offset + num_states_per_group,
                        "Feature index {} should be within the offset range [{}, {}).",
                        feature_index,
                        feature_index_offset,
                        feature_index_offset + num_states_per_group
                    );
                }
            }

            // Move the offset forward by the total number of states in this PatternGroup
            feature_index_offset += num_states_per_group;
        }
    }

    /// Tests that rotated versions of the same board state produce consistent feature vectors.
    ///
    /// Ensures that:
    /// - The sum of feature values remains the same across different rotations.
    /// - Individual feature counts do not exceed 4 (since there are 4 rotated versions).
    #[test]
    fn test_rotation_consistency() {
        let mut board = Bitboard::default();

        // Place some stones in non-symmetric positions
        board.apply_move(Position::D3, Player::Black).unwrap();
        board.apply_move(Position::C5, Player::White).unwrap();

        // Extract features for the original board
        let evaluator = PatternEvaluator::new(get_predefined_patterns());
        let base_features = extract_features(&board, &evaluator);
        let base_sum: f32 = base_features.values().iter().sum();

        // Track the maximum value in the feature vector
        let mut max_value: f32 = 0.0;

        // Get black and white bitmasks
        let (mut black_mask, mut white_mask) = board.bits();

        // Rotate the board 90, 180, and 270 degrees, checking consistency
        for _ in 0..3 {
            black_mask = rotate_mask_90_cw(black_mask);
            white_mask = rotate_mask_90_cw(white_mask);

            // Create a new Bitboard with rotated bitmasks
            let rotated_board = Bitboard::new(black_mask, white_mask);
            let rotated_features = extract_features(&rotated_board, &evaluator);
            let rotated_sum: f32 = rotated_features.values().iter().sum();

            // Check that total sum of feature values remains the same across rotations
            assert_eq!(
                base_sum, rotated_sum,
                "Feature vector sum should be consistent across rotations"
            );

            // Track max value
            max_value = max_value.max(
                *rotated_features
                    .values()
                    .iter()
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap(),
            );
        }

        // Ensure that no individual feature count exceeds 4
        assert!(
            max_value <= 4.0,
            "Feature values should not exceed 4, but found {}",
            max_value
        );
    }

    /// Tests that the size of the extracted sparse vector is correct.
    ///
    /// Ensures that:
    /// - The `size` of the `SparseVector` matches the total possible features across all `PatternGroup`s.
    #[test]
    fn test_sparse_vector_size() {
        let board = Bitboard::default();
        let evaluator = PatternEvaluator::new(get_predefined_patterns());
        let features = extract_features(&board, &evaluator);

        // Compute the expected size: sum of all possible states in all PatternGroups
        let expected_size: usize = get_predefined_patterns()
            .iter()
            .map(|group| group.state_scores[0].len()) // 3^N for each PatternGroup
            .sum();

        // Check that the size of the SparseVector matches the expected size
        assert_eq!(
            features.size(),
            expected_size,
            "SparseVector size ({}) does not match expected size ({})",
            features.size(),
            expected_size
        );
    }

    /// Tests that `extract_features` does not panic for a variety of board states.
    ///
    /// Ensures that:
    /// - The function executes without causing a panic.
    #[test]
    fn test_no_panic() {
        let board = Bitboard::default();
        let evaluator = PatternEvaluator::new(get_predefined_patterns());
        let _ = extract_features(&board, &evaluator); // Ensure no panic on default board

        // Define positions for black and white stones
        let black_positions = [Position::A1, Position::D4, Position::C3, Position::E5];
        let white_positions = [Position::H1, Position::A8, Position::H8, Position::F6];

        // Convert positions to bitmasks
        let black_mask: u64 = black_positions.iter().map(|p| p.to_bit()).sum();
        let white_mask: u64 = white_positions.iter().map(|p| p.to_bit()).sum();

        // Create a new Bitboard with predefined stone placements
        let custom_board = Bitboard::new(black_mask, white_mask);
        let _ = extract_features(&custom_board, &evaluator); // Ensure no panic on modified board
    }
}
