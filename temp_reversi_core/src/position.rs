use std::{fmt, str::FromStr};

/// Represents a position on the board with a row and column index.
/// Both `row` and `col` are 0-based indices, ranging from 0 to 7.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub row: usize, // Row index (0-7)
    pub col: usize, // Column index (0-7)
}

impl Position {
    /// Converts a `Position` into a `u64` bitmask representation for the bitboard.
    ///
    /// # Returns
    /// A `u64` value where a single bit represents the position.
    ///
    /// # Examples
    /// ```
    /// let pos = Position { row: 0, col: 0 };
    /// assert_eq!(pos.to_bit(), 1); // A1
    /// ```
    pub fn to_bit(&self) -> u64 {
        1 << (self.row * 8 + self.col)
    }

    /// Converts a `u64` bitmask back into a `Position`.
    ///
    /// # Arguments
    /// * `bit` - A `u64` value with exactly one bit set.
    ///
    /// # Returns
    /// Returns `Some(Position)` if the bitmask contains exactly one bit.
    /// Returns `None` if the bitmask is invalid (e.g., multiple bits are set or no bits are set).
    ///
    /// # Examples
    /// ```
    /// let bit = 1 << 28; // E5
    /// let pos = Position::from_bit(bit).unwrap();
    /// assert_eq!(pos, Position { row: 3, col: 4 });
    /// ```
    pub fn from_bit(bit: u64) -> Option<Self> {
        if bit.count_ones() != 1 {
            return None; // Invalid if not exactly one bit is set
        }
        let index = bit.trailing_zeros() as usize;
        Some(Position {
            row: index / 8,
            col: index % 8,
        })
    }

    /// Rotates the position 90 degrees clockwise.
    ///
    /// # Examples
    /// ```
    /// let mut pos = Position { row: 0, col: 0 }; // A1
    /// pos.rotate_90();
    /// assert_eq!(pos, Position { row: 0, col: 7 }); // H1
    /// ```
    pub fn rotate_90(&mut self) {
        let rotated = self.rotated_90();
        self.row = rotated.row;
        self.col = rotated.col;
    }

    /// Returns a new position rotated 90 degrees clockwise.
    ///
    /// # Examples
    /// ```
    /// let pos = Position { row: 0, col: 0 }; // A1
    /// let rotated_pos = pos.rotated_90();
    /// assert_eq!(rotated_pos, Position { row: 0, col: 7 }); // H1
    /// ```
    pub fn rotated_90(&self) -> Self {
        Position {
            row: self.col,
            col: 7 - self.row,
        }
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
    /// assert_eq!(position.row, 0);
    /// assert_eq!(position.col, 0);
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
        Ok(Position {
            row: (row as u8 - b'1') as usize,
            col: (col as u8 - b'A') as usize,
        })
    }
}
impl fmt::Display for Position {
    /// Formats a `Position` as a string in the format "A1".
    ///
    /// # Examples
    /// ```
    /// let pos = Position { row: 0, col: 0 };
    /// assert_eq!(format!("{}", pos), "A1");
    /// let pos = Position { row: 7, col: 7 };
    /// assert_eq!(format!("{}", pos), "H8");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let col = (self.col as u8 + b'A') as char; // Convert column to 'A'-'H'
        let row = (self.row as u8 + b'1') as char; // Convert row to '1'-'8'
        write!(f, "{}{}", col, row)
    }
}

/// Constants representing all positions on the board.
/// Each constant corresponds to a unique position indexed by row and column.
pub const A1: Position = Position { row: 0, col: 0 };
pub const A2: Position = Position { row: 1, col: 0 };
pub const A3: Position = Position { row: 2, col: 0 };
pub const A4: Position = Position { row: 3, col: 0 };
pub const A5: Position = Position { row: 4, col: 0 };
pub const A6: Position = Position { row: 5, col: 0 };
pub const A7: Position = Position { row: 6, col: 0 };
pub const A8: Position = Position { row: 7, col: 0 };

pub const B1: Position = Position { row: 0, col: 1 };
pub const B2: Position = Position { row: 1, col: 1 };
pub const B3: Position = Position { row: 2, col: 1 };
pub const B4: Position = Position { row: 3, col: 1 };
pub const B5: Position = Position { row: 4, col: 1 };
pub const B6: Position = Position { row: 5, col: 1 };
pub const B7: Position = Position { row: 6, col: 1 };
pub const B8: Position = Position { row: 7, col: 1 };

pub const C1: Position = Position { row: 0, col: 2 };
pub const C2: Position = Position { row: 1, col: 2 };
pub const C3: Position = Position { row: 2, col: 2 };
pub const C4: Position = Position { row: 3, col: 2 };
pub const C5: Position = Position { row: 4, col: 2 };
pub const C6: Position = Position { row: 5, col: 2 };
pub const C7: Position = Position { row: 6, col: 2 };
pub const C8: Position = Position { row: 7, col: 2 };

pub const D1: Position = Position { row: 0, col: 3 };
pub const D2: Position = Position { row: 1, col: 3 };
pub const D3: Position = Position { row: 2, col: 3 };
pub const D4: Position = Position { row: 3, col: 3 };
pub const D5: Position = Position { row: 4, col: 3 };
pub const D6: Position = Position { row: 5, col: 3 };
pub const D7: Position = Position { row: 6, col: 3 };
pub const D8: Position = Position { row: 7, col: 3 };

pub const E1: Position = Position { row: 0, col: 4 };
pub const E2: Position = Position { row: 1, col: 4 };
pub const E3: Position = Position { row: 2, col: 4 };
pub const E4: Position = Position { row: 3, col: 4 };
pub const E5: Position = Position { row: 4, col: 4 };
pub const E6: Position = Position { row: 5, col: 4 };
pub const E7: Position = Position { row: 6, col: 4 };
pub const E8: Position = Position { row: 7, col: 4 };

pub const F1: Position = Position { row: 0, col: 5 };
pub const F2: Position = Position { row: 1, col: 5 };
pub const F3: Position = Position { row: 2, col: 5 };
pub const F4: Position = Position { row: 3, col: 5 };
pub const F5: Position = Position { row: 4, col: 5 };
pub const F6: Position = Position { row: 5, col: 5 };
pub const F7: Position = Position { row: 6, col: 5 };
pub const F8: Position = Position { row: 7, col: 5 };

pub const G1: Position = Position { row: 0, col: 6 };
pub const G2: Position = Position { row: 1, col: 6 };
pub const G3: Position = Position { row: 2, col: 6 };
pub const G4: Position = Position { row: 3, col: 6 };
pub const G5: Position = Position { row: 4, col: 6 };
pub const G6: Position = Position { row: 5, col: 6 };
pub const G7: Position = Position { row: 6, col: 6 };
pub const G8: Position = Position { row: 7, col: 6 };

pub const H1: Position = Position { row: 0, col: 7 };
pub const H2: Position = Position { row: 1, col: 7 };
pub const H3: Position = Position { row: 2, col: 7 };
pub const H4: Position = Position { row: 3, col: 7 };
pub const H5: Position = Position { row: 4, col: 7 };
pub const H6: Position = Position { row: 5, col: 7 };
pub const H7: Position = Position { row: 6, col: 7 };
pub const H8: Position = Position { row: 7, col: 7 };

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_bit() {
        // Test converting positions to bitboard representation
        let pos = Position { row: 0, col: 0 }; // A1
        assert_eq!(pos.to_bit(), 1);

        let pos = Position { row: 7, col: 7 }; // H8
        assert_eq!(pos.to_bit(), 1 << 63);

        let pos = Position { row: 3, col: 4 }; // E5
        assert_eq!(pos.to_bit(), 1 << 28);
    }

    #[test]
    fn test_from_bit() {
        // Test converting bitboard representation back to positions
        assert_eq!(Position::from_bit(1), Some(Position { row: 0, col: 0 })); // A1
        assert_eq!(
            Position::from_bit(1 << 63),
            Some(Position { row: 7, col: 7 })
        ); // H8
        assert_eq!(
            Position::from_bit(1 << 28),
            Some(Position { row: 3, col: 4 })
        ); // E5
        assert_eq!(Position::from_bit(0), None); // Invalid bitmask
        assert_eq!(Position::from_bit(3), None); // Multiple bits set
    }

    #[test]
    fn test_constants() {
        // Test predefined constants for specific positions
        assert_eq!(A1.to_bit(), 1);
        assert_eq!(A2.to_bit(), 1 << 8);
        assert_eq!(H8.to_bit(), 1 << 63);
    }

    #[test]
    fn test_round_trip_conversion() {
        // Test round-trip conversion between Position and bitboard
        let pos = Position { row: 5, col: 6 }; // F6
        let bit = pos.to_bit();
        let converted_pos = Position::from_bit(bit).unwrap();
        assert_eq!(pos, converted_pos);
    }
}
