use std::fmt;
use std::ops::BitOr;
use std::str::FromStr;

/// Represents a position on the board with an internal bitboard representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    bit: u64, // Internal representation as a bitboard
}

impl Position {
    /// Creates a new `Position` from the given row and column.
    ///
    /// # Arguments
    ///
    /// * `row` - The row index (0-based, must be less than 8).
    /// * `col` - The column index (0-based, must be less than 8).
    ///
    /// # Panics
    ///
    /// Panics if `row` or `col` is not within the range 0..8.
    pub fn new(row: usize, col: usize) -> Self {
        assert!(row < 8 && col < 8, "row and col must be within 0..8");
        Self {
            bit: 1u64 << (row * 8 + col),
        }
    }

    /// Creates a `Position` from a given bitboard representation.
    ///
    /// # Arguments
    ///
    /// * `bit` - A `u64` representing the bitboard, where exactly one bit is set.
    ///
    /// # Returns
    ///
    /// Returns `Some(Position)` if the bitboard has exactly one bit set, otherwise `None`.
    pub fn from_bit(bit: u64) -> Option<Self> {
        if bit.count_ones() == 1 {
            Some(Self { bit })
        } else {
            None // Invalid if more than one bit is set
        }
    }

    /// Returns the row and column indices of the position.
    ///
    /// # Returns
    ///
    /// A tuple `(row, col)` where `row` and `col` are 0-based indices.
    pub fn to_row_col(&self) -> (usize, usize) {
        let index = self.bit.trailing_zeros() as usize;
        (index / 8, index % 8)
    }

    /// Returns the internal bitboard representation of the position.
    ///
    /// # Returns
    ///
    /// A `u64` representing the position as a single bit in a bitboard.
    pub fn to_bit(&self) -> u64 {
        self.bit
    }
}

/// Implements the `BitOr` trait to allow combining multiple positions.
impl BitOr for Position {
    type Output = u64;

    /// Combines two `Position` instances into a single bitboard by performing a bitwise OR operation.
    ///
    /// # Arguments
    ///
    /// * `self` - The first position.
    /// * `rhs` - The second position.
    ///
    /// # Returns
    ///
    /// A `u64` representing the combined bitboard of the two positions.
    fn bitor(self, rhs: Self) -> Self::Output {
        self.bit | rhs.bit
    }
}

/// Implements the `BitOr` trait to allow combining `u64` with `Position`.
impl BitOr<Position> for u64 {
    type Output = u64;

    /// Combines a `u64` bitboard with a `Position` by performing a bitwise OR operation.
    ///
    /// # Arguments
    ///
    /// * `self` - The first bitboard (`u64`).
    /// * `rhs` - The `Position` to combine.
    ///
    /// # Returns
    ///
    /// A `u64` representing the combined bitboard.
    fn bitor(self, rhs: Position) -> Self::Output {
        self | rhs.bit
    }
}

/// Implements the `BitOr` trait to allow combining `Position` with `u64`.
impl BitOr<u64> for Position {
    type Output = u64;

    /// Combines a `Position` with a `u64` bitboard by performing a bitwise OR operation.
    ///
    /// # Arguments
    ///
    /// * `self` - The `Position` to combine.
    /// * `rhs` - The `u64` bitboard.
    ///
    /// # Returns
    ///
    /// A `u64` representing the combined bitboard.
    fn bitor(self, rhs: u64) -> Self::Output {
        self.bit | rhs
    }
}

impl FromStr for Position {
    type Err = String;

    /// Converts a string in the format "A1" to a `Position`.
    ///
    /// # Arguments
    /// * `s` - A string slice representing the position on the board (e.g., "A1").
    ///
    /// # Returns
    /// Returns a `Position` object if the input is valid. Otherwise, it returns an error
    /// message as a `String`.
    ///
    /// # Errors
    /// This function returns an error if:
    /// - The input is not exactly two characters long.
    /// - The input does not represent a valid position on the board (e.g., out of range).
    ///
    /// # Examples
    /// ```
    /// use temp_reversi_core::Position;
    /// use std::str::FromStr;
    ///
    /// let position = Position::from_str("A1").unwrap();
    /// assert_eq!(position.to_row_col(), (0, 0));
    ///
    /// let invalid_position = Position::from_str("Z9");
    /// assert!(invalid_position.is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Check if the input string length is exactly 2
        if s.len() != 2 {
            return Err("Invalid format. Expected format: A1".to_string());
        }

        // Extract the column (A-H) and row (1-8) from the string
        let col = s.chars().nth(0).unwrap().to_ascii_uppercase();
        let row = s.chars().nth(1).unwrap();

        // Validate the column and row range
        if !('A'..='H').contains(&col) || !('1'..='8').contains(&row) {
            return Err("Position out of range. Valid range: A1 to H8".to_string());
        }

        // Convert the column and row to indices
        let row_index = (row as u8 - b'1') as usize;
        let col_index = (col as u8 - b'A') as usize;

        Ok(Position::new(row_index, col_index))
    }
}

impl fmt::Display for Position {
    /// Formats a `Position` as a string in the format "A1".
    ///
    /// # Examples
    /// ```
    /// use temp_reversi_core::Position;
    ///
    /// let pos = Position::new(0, 0);
    /// assert_eq!(format!("{}", pos), "A1");
    ///
    /// let pos = Position::new(7, 7);
    /// assert_eq!(format!("{}", pos), "H8");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (row, col) = self.to_row_col();
        let col_char = (col as u8 + b'A') as char; // Convert column to 'A'-'H'
        let row_char = (row as u8 + b'1') as char; // Convert row to '1'-'8'
        write!(f, "{}{}", col_char, row_char)
    }
}

/// Constants representing all positions on the board.
/// Each constant corresponds to a unique position indexed by row and column.
pub const A1: Position = Position {
    bit: 1u64 << (0 * 8 + 0),
};
pub const A2: Position = Position {
    bit: 1u64 << (1 * 8 + 0),
};
pub const A3: Position = Position {
    bit: 1u64 << (2 * 8 + 0),
};
pub const A4: Position = Position {
    bit: 1u64 << (3 * 8 + 0),
};
pub const A5: Position = Position {
    bit: 1u64 << (4 * 8 + 0),
};
pub const A6: Position = Position {
    bit: 1u64 << (5 * 8 + 0),
};
pub const A7: Position = Position {
    bit: 1u64 << (6 * 8 + 0),
};
pub const A8: Position = Position {
    bit: 1u64 << (7 * 8 + 0),
};

pub const B1: Position = Position {
    bit: 1u64 << (0 * 8 + 1),
};
pub const B2: Position = Position {
    bit: 1u64 << (1 * 8 + 1),
};
pub const B3: Position = Position {
    bit: 1u64 << (2 * 8 + 1),
};
pub const B4: Position = Position {
    bit: 1u64 << (3 * 8 + 1),
};
pub const B5: Position = Position {
    bit: 1u64 << (4 * 8 + 1),
};
pub const B6: Position = Position {
    bit: 1u64 << (5 * 8 + 1),
};
pub const B7: Position = Position {
    bit: 1u64 << (6 * 8 + 1),
};
pub const B8: Position = Position {
    bit: 1u64 << (7 * 8 + 1),
};

pub const C1: Position = Position {
    bit: 1u64 << (0 * 8 + 2),
};
pub const C2: Position = Position {
    bit: 1u64 << (1 * 8 + 2),
};
pub const C3: Position = Position {
    bit: 1u64 << (2 * 8 + 2),
};
pub const C4: Position = Position {
    bit: 1u64 << (3 * 8 + 2),
};
pub const C5: Position = Position {
    bit: 1u64 << (4 * 8 + 2),
};
pub const C6: Position = Position {
    bit: 1u64 << (5 * 8 + 2),
};
pub const C7: Position = Position {
    bit: 1u64 << (6 * 8 + 2),
};
pub const C8: Position = Position {
    bit: 1u64 << (7 * 8 + 2),
};

pub const D1: Position = Position {
    bit: 1u64 << (0 * 8 + 3),
};
pub const D2: Position = Position {
    bit: 1u64 << (1 * 8 + 3),
};
pub const D3: Position = Position {
    bit: 1u64 << (2 * 8 + 3),
};
pub const D4: Position = Position {
    bit: 1u64 << (3 * 8 + 3),
};
pub const D5: Position = Position {
    bit: 1u64 << (4 * 8 + 3),
};
pub const D6: Position = Position {
    bit: 1u64 << (5 * 8 + 3),
};
pub const D7: Position = Position {
    bit: 1u64 << (6 * 8 + 3),
};
pub const D8: Position = Position {
    bit: 1u64 << (7 * 8 + 3),
};

pub const E1: Position = Position {
    bit: 1u64 << (0 * 8 + 4),
};
pub const E2: Position = Position {
    bit: 1u64 << (1 * 8 + 4),
};
pub const E3: Position = Position {
    bit: 1u64 << (2 * 8 + 4),
};
pub const E4: Position = Position {
    bit: 1u64 << (3 * 8 + 4),
};
pub const E5: Position = Position {
    bit: 1u64 << (4 * 8 + 4),
};
pub const E6: Position = Position {
    bit: 1u64 << (5 * 8 + 4),
};
pub const E7: Position = Position {
    bit: 1u64 << (6 * 8 + 4),
};
pub const E8: Position = Position {
    bit: 1u64 << (7 * 8 + 4),
};

pub const F1: Position = Position {
    bit: 1u64 << (0 * 8 + 5),
};
pub const F2: Position = Position {
    bit: 1u64 << (1 * 8 + 5),
};
pub const F3: Position = Position {
    bit: 1u64 << (2 * 8 + 5),
};
pub const F4: Position = Position {
    bit: 1u64 << (3 * 8 + 5),
};
pub const F5: Position = Position {
    bit: 1u64 << (4 * 8 + 5),
};
pub const F6: Position = Position {
    bit: 1u64 << (5 * 8 + 5),
};
pub const F7: Position = Position {
    bit: 1u64 << (6 * 8 + 5),
};
pub const F8: Position = Position {
    bit: 1u64 << (7 * 8 + 5),
};

pub const G1: Position = Position {
    bit: 1u64 << (0 * 8 + 6),
};
pub const G2: Position = Position {
    bit: 1u64 << (1 * 8 + 6),
};
pub const G3: Position = Position {
    bit: 1u64 << (2 * 8 + 6),
};
pub const G4: Position = Position {
    bit: 1u64 << (3 * 8 + 6),
};
pub const G5: Position = Position {
    bit: 1u64 << (4 * 8 + 6),
};
pub const G6: Position = Position {
    bit: 1u64 << (5 * 8 + 6),
};
pub const G7: Position = Position {
    bit: 1u64 << (6 * 8 + 6),
};
pub const G8: Position = Position {
    bit: 1u64 << (7 * 8 + 6),
};

pub const H1: Position = Position {
    bit: 1u64 << (0 * 8 + 7),
};
pub const H2: Position = Position {
    bit: 1u64 << (1 * 8 + 7),
};
pub const H3: Position = Position {
    bit: 1u64 << (2 * 8 + 7),
};
pub const H4: Position = Position {
    bit: 1u64 << (3 * 8 + 7),
};
pub const H5: Position = Position {
    bit: 1u64 << (4 * 8 + 7),
};
pub const H6: Position = Position {
    bit: 1u64 << (5 * 8 + 7),
};
pub const H7: Position = Position {
    bit: 1u64 << (6 * 8 + 7),
};
pub const H8: Position = Position {
    bit: 1u64 << (7 * 8 + 7),
};

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the creation of a `Position` using row and column.
    #[test]
    fn test_position_creation() {
        let pos = Position::new(0, 0); // A1
        assert_eq!(pos.to_bit(), 1u64 << 0);
        assert_eq!(format!("{}", pos), "A1");
    }

    /// Tests the `BitOr` operator for combining multiple positions.
    #[test]
    fn test_bitor_operator() {
        let p1 = Position::new(0, 0); // A1
        let p2 = Position::new(0, 1); // B1
        let p3 = Position::new(1, 0); // A2

        let pattern = p1 | p2 | p3;
        assert_eq!(pattern, (1u64 << 0) | (1u64 << 1) | (1u64 << 8));
    }

    /// Tests the conversion of a `Position` to its row and column indices.
    #[test]
    fn test_to_row_col() {
        let pos = Position::new(3, 5); // F4
        assert_eq!(pos.to_row_col(), (3, 5));
    }

    /// Tests creating a `Position` from a bitboard representation.
    #[test]
    fn test_from_bit() {
        let bit = 1u64 << 0; // A1
        let pos = Position::from_bit(bit).unwrap();
        assert_eq!(pos.to_row_col(), (0, 0));
    }

    /// Tests creating a `Position` from a string.
    #[test]
    fn test_from_str() {
        let pos = Position::from_str("A1").unwrap();
        assert_eq!(pos.to_row_col(), (0, 0));

        let pos = Position::from_str("H8").unwrap();
        assert_eq!(pos.to_row_col(), (7, 7));

        assert!(Position::from_str("Z9").is_err());
        assert!(Position::from_str("AA").is_err());
    }

    /// Tests the `Display` implementation for `Position`.
    #[test]
    fn test_display() {
        let pos = Position::new(7, 7); // H8
        assert_eq!(format!("{}", pos), "H8");
    }
}
