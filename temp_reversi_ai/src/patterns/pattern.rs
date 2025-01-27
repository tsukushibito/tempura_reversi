use temp_reversi_core::{utils::rotate_mask_90_clockwise, Position};

/// Represents a pattern on the Reversi board.
///
/// Patterns are represented using a 64-bit bitmask, where each bit corresponds to a cell on the 8x8 board.
/// This struct supports pattern creation, rotation, and property calculation (e.g., cell count and state count).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pattern {
    /// The bitmask representing the pattern's positions.
    pub board_mask: u64,
    /// A human-readable name for the pattern (optional, useful for debugging).
    pub name: Option<String>,
}

impl Pattern {
    /// Creates a new pattern with the given board mask and an optional name.
    ///
    /// # Arguments
    ///
    /// * `board_mask` - A 64-bit bitmask representing the pattern's positions.
    /// * `name` - An optional name for the pattern.
    ///
    /// # Returns
    ///
    /// A new `Pattern` instance.
    pub fn new(board_mask: u64, name: Option<&str>) -> Self {
        Self {
            board_mask,
            name: name.map(|s| s.to_string()),
        }
    }

    /// Creates a pattern from a list of `Position`s.
    ///
    /// # Arguments
    ///
    /// * `positions` - A slice of `Position` values representing the cells in the pattern.
    /// * `name` - An optional name for the pattern.
    ///
    /// # Returns
    ///
    /// A new `Pattern` instance.
    pub fn from_positions(positions: &[Position], name: Option<&str>) -> Self {
        let mut board_mask = 0u64;
        for pos in positions {
            board_mask |= pos.to_bit();
        }
        Self::new(board_mask, name)
    }

    /// Calculates the number of cells included in this pattern.
    ///
    /// # Returns
    ///
    /// The number of cells in the pattern.
    pub fn cell_count(&self) -> usize {
        self.board_mask.count_ones() as usize
    }

    /// Calculates the total number of states for this pattern.
    ///
    /// Each cell can have 3 states: empty, black, or white.
    /// The total number of states is `3^(number of cells)`.
    ///
    /// # Returns
    ///
    /// The total number of possible states for the pattern.
    pub fn state_count(&self) -> usize {
        3_usize.pow(self.cell_count() as u32)
    }

    /// Rotates the pattern's bitmask 90 degrees clockwise.
    ///
    /// # Returns
    ///
    /// A new `Pattern` instance representing the rotated pattern.
    pub fn rotated_90_clockwise(&self) -> Self {
        let rotated_mask = rotate_mask_90_clockwise(self.board_mask);
        Self::new(rotated_mask, self.name.clone().as_deref())
    }

    /// Rotates the pattern's bitmask 180 degrees clockwise.
    ///
    /// # Returns
    ///
    /// A new `Pattern` instance representing the rotated pattern.
    pub fn rotated_180_clockwise(&self) -> Self {
        self.rotated_90_clockwise().rotated_90_clockwise()
    }

    /// Rotates the pattern's bitmask 270 degrees clockwise.
    ///
    /// # Returns
    ///
    /// A new `Pattern` instance representing the rotated pattern.
    pub fn rotated_270_clockwise(&self) -> Self {
        self.rotated_90_clockwise()
            .rotated_90_clockwise()
            .rotated_90_clockwise()
    }

    /// Returns all rotations of the pattern: 0, 90, 180, and 270 degrees.
    ///
    /// # Returns
    ///
    /// A vector of `Pattern` instances representing all rotations of the pattern.
    pub fn all_rotations(&self) -> Vec<Self> {
        vec![
            self.clone(),
            self.rotated_90_clockwise(),
            self.rotated_180_clockwise(),
            self.rotated_270_clockwise(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use temp_reversi_core::Position;

    #[test]
    fn test_pattern_creation() {
        let positions = vec![
            Position::new(0, 0), // A1
            Position::new(0, 7), // H1
            Position::new(7, 0), // A8
            Position::new(7, 7), // H8
        ];
        let pattern = Pattern::from_positions(&positions, Some("Corner"));
        assert_eq!(pattern.cell_count(), 4);
        assert_eq!(pattern.state_count(), 81); // 3^4
        assert_eq!(pattern.board_mask, 0x8100000000000081);
    }

    #[test]
    fn test_rotated_90_clockwise() {
        let mask = 0x0100000000000000; // A8
        let pattern = Pattern::new(mask, Some("Test"));

        let rotated = pattern.rotated_90_clockwise();
        assert_eq!(rotated.board_mask, 0x0000000000000001); // A1
    }

    #[test]
    fn test_rotated_180_clockwise() {
        let mask = 0x0100000000000000; // A8
        let pattern = Pattern::new(mask, Some("Test"));

        let rotated = pattern.rotated_180_clockwise();
        assert_eq!(rotated.board_mask, 0x0000000000000080); // H1
    }

    #[test]
    fn test_rotated_270_clockwise() {
        let mask = 0x0100000000000000; // A8
        let pattern = Pattern::new(mask, Some("Test"));

        let rotated = pattern.rotated_270_clockwise();
        assert_eq!(rotated.board_mask, 0x8000000000000000); // H8
    }

    #[test]
    fn test_all_rotations() {
        let mask = 0x0100000000000000; // A8
        let pattern = Pattern::new(mask, Some("Test"));

        let rotations = pattern.all_rotations();
        assert_eq!(rotations.len(), 4);
        assert_eq!(rotations[0].board_mask, 0x0100000000000000); // Original
        assert_eq!(rotations[1].board_mask, 0x0000000000000001); // 90 degrees
        assert_eq!(rotations[2].board_mask, 0x0000000000000080); // 180 degrees
        assert_eq!(rotations[3].board_mask, 0x8000000000000000); // 270 degrees
    }
}
