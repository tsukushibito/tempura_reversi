use std::sync::Arc;
use temp_reversi_core::utils::rotate_mask_90_clockwise;

/// A group of patterns representing a base pattern and its rotations (0°, 90°, 180°, 270°).
///
/// Each group shares a common set of state scores, where each state corresponds
/// to a specific configuration of the cells in the pattern.
pub struct PatternGroup {
    /// A vector of the 4 rotation masks (0°, 90°, 180°, 270°).
    pub rotation_masks: Vec<u64>,
    /// A shared vector of scores for each state of the pattern (3^cell_count).
    pub state_scores: Arc<Vec<i32>>,
    /// A human-readable name for the pattern group (optional).
    pub name: Option<String>,
}

impl PatternGroup {
    /// Creates a new PatternGroup from a single base pattern.
    ///
    /// # Arguments
    ///
    /// * `base_pattern` - The base bitmask for the pattern.
    /// * `state_scores` - A vector of scores for all states of the pattern.
    /// * `name` - An optional name for the pattern group.
    ///
    /// # Panics
    ///
    /// Panics if `state_scores` does not match the expected number of states (`3^cell_count`).
    pub fn new(base_pattern: u64, state_scores: Vec<i32>, name: Option<&str>) -> Self {
        // Generate all rotation masks using rotate_mask_90_clockwise
        let rotation_masks = vec![
            base_pattern,
            rotate_mask_90_clockwise(base_pattern),
            rotate_mask_90_clockwise(rotate_mask_90_clockwise(base_pattern)),
            rotate_mask_90_clockwise(rotate_mask_90_clockwise(rotate_mask_90_clockwise(
                base_pattern,
            ))),
        ];

        // Normalize to the smallest rotation for consistency
        let normalized = *rotation_masks.iter().min().unwrap();

        // Validate state_scores length
        let cell_count = normalized.count_ones() as usize;
        let expected_state_count = 3_usize.pow(cell_count as u32);
        assert_eq!(
            state_scores.len(),
            expected_state_count,
            "Invalid state_scores length"
        );

        Self {
            rotation_masks,
            state_scores: Arc::new(state_scores),
            name: name.map(|s| s.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use temp_reversi_core::utils::rotate_mask_90_clockwise;

    #[test]
    fn test_pattern_group_creation() {
        // Example base pattern: corners
        let base_pattern: u64 = 0x8100000000000081; // Corners (A1, H1, A8, H8)
        let cell_count = base_pattern.count_ones() as usize;
        let expected_state_count = 3_usize.pow(cell_count as u32);

        // State scores: initialize with dummy values
        let state_scores = vec![1; expected_state_count];

        // Create the PatternGroup
        let group = PatternGroup::new(base_pattern, state_scores.clone(), Some("Corners"));

        // Assert rotation masks
        let expected_rotations = vec![
            base_pattern,
            rotate_mask_90_clockwise(base_pattern),
            rotate_mask_90_clockwise(rotate_mask_90_clockwise(base_pattern)),
            rotate_mask_90_clockwise(rotate_mask_90_clockwise(rotate_mask_90_clockwise(
                base_pattern,
            ))),
        ];

        assert_eq!(group.rotation_masks, expected_rotations);

        // Assert state scores are shared
        assert_eq!(*group.state_scores, state_scores);

        // Assert name
        assert_eq!(group.name.as_deref(), Some("Corners"));
    }

    #[test]
    fn test_pattern_group_min_rotation() {
        // Base pattern and its rotations
        let base_pattern = 0x8100000000000081; // Corners
        let rotations = vec![
            base_pattern,
            rotate_mask_90_clockwise(base_pattern),
            rotate_mask_90_clockwise(rotate_mask_90_clockwise(base_pattern)),
            rotate_mask_90_clockwise(rotate_mask_90_clockwise(rotate_mask_90_clockwise(
                base_pattern,
            ))),
        ];

        // Normalize to the smallest rotation
        let normalized = *rotations.iter().min().unwrap();

        // Create PatternGroup
        let state_scores = vec![0; 3_usize.pow(base_pattern.count_ones() as u32)];
        let group = PatternGroup::new(base_pattern, state_scores, Some("Corners"));

        // Assert the normalized mask is the smallest
        assert_eq!(group.rotation_masks.iter().min().unwrap(), &normalized);
    }

    #[test]
    #[should_panic(expected = "Invalid state_scores length")]
    fn test_pattern_group_invalid_state_scores_length() {
        // Example base pattern: corners
        let base_pattern: u64 = 0x8100000000000081; // Corners
        let cell_count = base_pattern.count_ones() as usize;
        let invalid_state_count = 3_usize.pow(cell_count as u32) - 1; // Incorrect length

        // State scores: invalid length
        let state_scores = vec![0; invalid_state_count];

        // This should panic due to invalid state_scores length
        PatternGroup::new(base_pattern, state_scores, Some("Invalid"));
    }

    #[test]
    fn test_rotation_masks_consistency() {
        // Example base pattern: corners
        let base_pattern: u64 = 0x00000000000000FF; // Row 1
        let state_scores = vec![0; 3_usize.pow(base_pattern.count_ones() as u32)];

        // Create the PatternGroup
        let group = PatternGroup::new(base_pattern, state_scores, Some("Row 1"));

        // Manually compute the expected rotations
        let expected_rotations = vec![
            base_pattern,
            rotate_mask_90_clockwise(base_pattern),
            rotate_mask_90_clockwise(rotate_mask_90_clockwise(base_pattern)),
            rotate_mask_90_clockwise(rotate_mask_90_clockwise(rotate_mask_90_clockwise(
                base_pattern,
            ))),
        ];

        assert_eq!(group.rotation_masks, expected_rotations);
    }
}
