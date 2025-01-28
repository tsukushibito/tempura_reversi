use std::collections::HashMap;

/// Represents a single rotated pattern and its key-to-index mapping.
///
/// A `Pattern` contains a bitmask representing a specific rotation of a pattern
/// and a precomputed mapping from masked board states (black and white stones)
/// to their corresponding state indices.
pub struct Pattern {
    /// Bitmask representing the rotated pattern.
    pub mask: u64,
    /// Mapping from masked board state (black and white stones) to state index.
    pub key_to_index: HashMap<(u64, u64), usize>,
}

impl Pattern {
    /// Creates a new `Pattern` with a precomputed key-to-index map.
    ///
    /// # Arguments
    /// * `mask` - A 64-bit integer representing the bitmask of the pattern.
    ///
    /// # Returns
    /// A `Pattern` struct with the given bitmask and a precomputed key-to-index map.
    pub fn new(mask: u64) -> Self {
        let key_to_index = Self::precompute_key_to_index(mask);
        Self { mask, key_to_index }
    }

    /// Precomputes the key-to-index mapping for a given mask.
    ///
    /// This function calculates all possible board states (black and white stones)
    /// for the bits set in the given mask and maps them to their respective state indices.
    ///
    /// # Arguments
    /// * `mask` - A 64-bit integer representing the bitmask of the pattern.
    ///
    /// # Returns
    /// A `HashMap` mapping pairs of masked board states (black, white) to state indices.
    fn precompute_key_to_index(mask: u64) -> HashMap<(u64, u64), usize> {
        let mut mapping = HashMap::new();

        // Collect positions of bits that are set in the mask
        let cell_positions: Vec<u64> = (0..64).filter(|&i| mask & (1 << i) != 0).collect();

        // Calculate the number of cells covered by the mask
        let num_cells = cell_positions.len();

        // There are 3^num_cells possible states (0: Empty, 1: Black, 2: White)
        let num_states = 3_usize.pow(num_cells as u32);

        // Generate all possible states and compute the corresponding masked board state
        for state_index in 0..num_states {
            let mut masked_black = 0;
            let mut masked_white = 0;
            let mut state = state_index;

            for &pos in &cell_positions {
                let cell_state = state % 3; // 0: Empty, 1: Black, 2: White
                state /= 3;

                match cell_state {
                    1 => masked_black |= 1 << pos, // Black stone
                    2 => masked_white |= 1 << pos, // White stone
                    _ => {}                        // Empty cell
                }
            }

            // Map the masked state to the state index
            mapping.insert((masked_black, masked_white), state_index);
        }

        mapping
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the precompute_key_to_index function to ensure correct mappings are generated.
    #[test]
    fn test_precompute_key_to_index() {
        let mask = 0b000000000_000000111_000000000_000000000_000000000; // A horizontal line in the third row
        let pattern = Pattern::new(mask);

        // Verify that the number of entries matches the expected number of states
        let expected_num_states = 3_usize.pow(3); // 3 cells covered by the mask
        assert_eq!(pattern.key_to_index.len(), expected_num_states);

        // Verify that a specific state is correctly mapped
        let black_mask = 0b000000000_000000101_000000000_000000000_000000000; // Black stones on the ends
        let white_mask = 0b000000000_000000010_000000000_000000000_000000000; // White stone in the middle

        let key = (black_mask & mask, white_mask & mask);
        assert!(pattern.key_to_index.contains_key(&key));

        let state_index = pattern.key_to_index[&key];
        assert_eq!(state_index, 16); // Corresponding to [Black, White, Black]
    }
}
