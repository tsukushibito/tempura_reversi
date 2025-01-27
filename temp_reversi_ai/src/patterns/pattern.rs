use temp_reversi_core::Position;

/// A struct to represent a pattern on the Reversi board.
///
/// This structure represents a specific pattern on the board using a bitmask.
/// Patterns can be used for evaluation purposes, such as detecting positions
/// like corners, edges, or other strategic areas of the board.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pattern {
    /// The bitmask representing the pattern's positions.
    ///
    /// Each bit in the 64-bit integer corresponds to a position on the 8x8 Reversi board.
    /// If a bit is set to 1, the position is part of the pattern.
    pub board_mask: u64,

    /// A human-readable name for the pattern (optional, for debugging purposes).
    ///
    /// Examples: "Corner", "Edge", "Inner".
    pub name: Option<String>,
}

impl Pattern {
    /// Creates a new pattern with the given board mask and an optional name.
    ///
    /// # Arguments
    ///
    /// * `board_mask` - A 64-bit integer representing the positions of the pattern on the board.
    /// * `name` - An optional name for the pattern, useful for debugging or identification.
    ///
    /// # Examples
    ///
    /// ```
    /// let corner_pattern = Pattern::new(0x8100000000000081, Some("Corner"));
    /// ```
    pub fn new(board_mask: u64, name: Option<&str>) -> Self {
        Self {
            board_mask,
            name: name.map(|s| s.to_string()),
        }
    }

    /// Creates a pattern from a list of `Position`s.
    ///
    /// This method generates a bitmask based on the provided positions and creates a pattern.
    ///
    /// # Arguments
    ///
    /// * `positions` - A slice of `Position`s representing the locations included in the pattern.
    /// * `name` - An optional name for the pattern, useful for debugging or identification.
    ///
    /// # Returns
    ///
    /// A new `Pattern` instance that represents the provided positions.
    ///
    /// # Examples
    ///
    /// ```
    /// use temp_reversi_core::position::Position;
    ///
    /// let positions = vec![
    ///     Position { row: 0, col: 0 }, // A1
    ///     Position { row: 0, col: 7 }, // H1
    ///     Position { row: 7, col: 0 }, // A8
    ///     Position { row: 7, col: 7 }, // H8
    /// ];
    /// let corner_pattern = Pattern::from_positions(&positions, Some("Corner"));
    /// assert_eq!(corner_pattern.board_mask, 0x8100000000000081);
    /// ```
    pub fn from_positions(positions: &[Position], name: Option<&str>) -> Self {
        let mut board_mask = 0u64;
        for pos in positions {
            board_mask |= pos.to_bit();
        }
        Self::new(board_mask, name)
    }
}
