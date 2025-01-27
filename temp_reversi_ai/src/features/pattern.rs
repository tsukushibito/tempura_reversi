use std::collections::HashMap;

use temp_reversi_core::{Bitboard, Position};

use crate::utils::SparseVector;

#[derive(Clone, Debug)]
pub struct Pattern {
    pub id: usize,
    pub pattern_bits: [u64; 4], // Bitboard representation for each rotation
}

impl Pattern {
    /// Creates a new pattern from a list of positions.
    ///
    /// # Arguments
    /// * `id` - The unique identifier for the pattern.
    /// * `positions` - A slice of `Position` objects representing the pattern.
    pub fn from_positions(id: usize, positions: &[Position]) -> Self {
        let mut pattern_bits = [0u64; 4];
        let mut positions = positions.to_vec();

        // Generate bitboard masks for each rotation (0, 90, 180, 270 degrees)
        pattern_bits.iter_mut().for_each(|bits| {
            for pos in &positions {
                *bits |= pos.to_bit(); // Use to_bit for bit calculation
            }
            positions.iter_mut().for_each(|p| p.rotate_90());
        });

        Self { id, pattern_bits }
    }

    /// Returns the number of states for a single rotation of the pattern.
    ///
    /// Each cell can have 3 states: empty, black, or white. The total number of states
    /// is calculated as 3 raised to the power of the number of bits in the first rotation.
    pub fn state_count_single_rotation(&self) -> usize {
        3usize.pow(self.pattern_bits[0].count_ones() as u32)
    }

    /// Calculates the state indices for all rotations based on the board state.
    ///
    /// # Arguments
    /// * `board` - A reference to the `Bitboard` representing the current game state.
    ///
    /// # Returns
    /// An array of state indices, one for each rotation.
    pub fn state_indices(&self, board: &Bitboard) -> [usize; 4] {
        let mut indices = [0usize; 4];

        for (i, index) in indices.iter_mut().enumerate() {
            let pattern = self.pattern_bits[i];
            let black_pattern = board.bits().0 & pattern;
            let white_pattern = board.bits().1 & pattern;

            // Calculate the state index for this rotation
            *index = Self::calculate_index(black_pattern, white_pattern, pattern);
        }

        indices
    }

    /// Helper function to calculate a single state index based on the bitboard patterns.
    ///
    /// # Arguments
    /// * `black_pattern` - The bitboard mask for black stones.
    /// * `white_pattern` - The bitboard mask for white stones.
    /// * `pattern` - The bitboard mask for the current rotation.
    ///
    /// # Returns
    /// The calculated state index for the given pattern.
    fn calculate_index(black_pattern: u64, white_pattern: u64, pattern: u64) -> usize {
        let mut idx = 0;
        let mut bit_pos = 0;
        let mut pattern_copy = pattern;

        while pattern_copy != 0 {
            let bit = pattern_copy & (!pattern_copy + 1); // Extract the lowest set bit
            let val = if (black_pattern & bit) != 0 {
                1 // Black stone
            } else if (white_pattern & bit) != 0 {
                2 // White stone
            } else {
                0 // Empty cell
            };

            idx += 3usize.pow(bit_pos) * val; // Update the index with the calculated value
            bit_pos += 1;
            pattern_copy &= pattern_copy - 1; // Clear the lowest set bit
        }

        idx
    }

    /// Computes the feature vector for the current pattern based on the board state.
    ///
    /// # Arguments
    /// * `board` - A reference to the `Bitboard` representing the current game state.
    ///
    /// # Returns
    /// A `SparseVector` containing the feature representation of the pattern.
    pub fn feature(&self, board: &Bitboard) -> SparseVector {
        let mut index_count: HashMap<usize, f32> = HashMap::new();

        // Count occurrences of each state index across all rotations
        for index in self.state_indices(board) {
            *index_count.entry(index).or_insert(0.0) += 1.0;
        }

        let mut indices = Vec::new();
        let mut values = Vec::new();

        for (index, value) in index_count {
            indices.push(index);
            values.push(value);
        }

        // Create a sparse vector from the collected indices and values
        SparseVector::new(indices, values, self.state_count_single_rotation()).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use temp_reversi_core::Position;

    #[test]
    fn test_from_positions() {
        let positions = vec![
            Position { row: 0, col: 0 },
            Position { row: 1, col: 0 },
            Position { row: 0, col: 1 },
        ];
        let pattern = Pattern::from_positions(1, &positions);

        let expected_bits = [
            0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001_0000_0011,
            0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000_0000_1100_0000,
            0b1100_0000_1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
            0b0000_0011_0000_0001_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        ];

        (0..4).for_each(|i| {
            assert_eq!(
                pattern.pattern_bits[i],
                expected_bits[i],
                "Pattern bits for rotation {} incorrect",
                i * 90
            );
        });
    }

    #[test]
    fn test_state_count_single_rotation() {
        let positions = vec![
            Position { row: 0, col: 0 },
            Position { row: 1, col: 0 },
            Position { row: 0, col: 1 },
        ];
        let pattern = Pattern::from_positions(1, &positions);
        assert_eq!(pattern.state_count_single_rotation(), 3 * 3 * 3); // 3^3 = 27
    }
}
