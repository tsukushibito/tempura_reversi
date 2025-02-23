use std::collections::HashMap;
use temp_reversi_core::utils::{rotate_mask_180, rotate_mask_270_ccw, rotate_mask_90_ccw};

/// Represents a pattern used for evaluating board positions in Reversi.
///
/// A `Pattern` consists of a bitmask defining a specific pattern on the board
/// and a precomputed mapping from board states to their corresponding indices.
#[derive(Debug, Clone)]
pub struct Pattern {
    /// Bitmask representing the pattern on the board.
    pub mask: u64,
    /// Mapping from masked board states (black and white stones) to state indices.
    pub key_to_index: HashMap<(u64, u64), usize>,
}

impl Pattern {
    /// Creates a new `Pattern` instance.
    ///
    /// # Arguments
    /// * `mask` - A 64-bit integer representing the bitmask of the pattern.
    /// * `base_pattern` - An optional reference to a base pattern and its rotation.
    ///
    /// If `base_pattern` is provided, the `key_to_index` mapping is derived
    /// from the base pattern by adjusting for rotation.
    ///
    /// # Returns
    /// A `Pattern` instance with a precomputed `key_to_index` mapping.
    pub fn new(mask: u64, base_pattern: Option<(&Pattern, u8)>) -> Self {
        let key_to_index = Self::precompute_key_to_index(mask, base_pattern);
        Self { mask, key_to_index }
    }

    /// Precomputes the key-to-index mapping for a given pattern.
    ///
    /// # Arguments
    /// * `mask` - A 64-bit integer representing the bitmask of the pattern.
    /// * `base_pattern` - An optional reference to a base pattern and its rotation.
    ///
    /// If `base_pattern` is provided, the board states are rotated back to
    /// the base orientation before retrieving their indices.
    ///
    /// # Returns
    /// A `HashMap` mapping masked board states `(black_mask, white_mask)` to state indices.
    fn precompute_key_to_index(
        mask: u64,
        base_pattern: Option<(&Pattern, u8)>,
    ) -> HashMap<(u64, u64), usize> {
        let mut mapping = HashMap::new();

        // Collect positions of bits set in the mask.
        let cell_positions: Vec<u64> = (0..64).filter(|&i| mask & (1 << i) != 0).collect();
        let num_cells = cell_positions.len();
        let num_states = 3_usize.pow(num_cells as u32);

        // Generate all possible masked board states.
        for state_index in 0..num_states {
            let mut masked_black = 0;
            let mut masked_white = 0;
            let mut state = state_index;

            for &pos in &cell_positions {
                let cell_state = state % 3;
                state /= 3;

                match cell_state {
                    1 => masked_black |= 1 << pos, // Black stone
                    2 => masked_white |= 1 << pos, // White stone
                    _ => {}                        // Empty cell
                }
            }

            if let Some((base, rotation)) = base_pattern {
                // Adjust rotation to match the base pattern.
                let (base_black, base_white) = match rotation {
                    1 => (
                        rotate_mask_90_ccw(masked_black),
                        rotate_mask_90_ccw(masked_white),
                    ), // 90-degree counterclockwise
                    2 => (rotate_mask_180(masked_black), rotate_mask_180(masked_white)), // 180-degree
                    3 => (
                        rotate_mask_270_ccw(masked_black),
                        rotate_mask_270_ccw(masked_white),
                    ), // 270-degree counterclockwise
                    _ => (masked_black, masked_white), // No rotation
                };

                // Retrieve the index from the base pattern's key-to-index mapping.
                if let Some(&base_index) = base.key_to_index.get(&(base_black, base_white)) {
                    mapping.insert((masked_black, masked_white), base_index);
                }
            } else {
                // Directly assign indices for the base pattern.
                mapping.insert((masked_black, masked_white), state_index);
            }
        }

        mapping
    }
}
