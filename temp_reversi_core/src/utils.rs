/// Utility functions for board manipulation and general operations.
///
/// This module provides helper functions such as bitboard rotations
/// that are useful for working with the 8x8 Reversi board.
///
/// Rotates a 64-bit bitmask 90 degrees clockwise.
///
/// This function is used to manipulate the bitboard representation of an 8x8 Reversi board.
/// Each bit represents a cell on the board, and the function calculates the rotated
/// bitmask after a 90-degree clockwise rotation.
///
/// # Arguments
///
/// * `mask` - A 64-bit bitmask representing the original board state.
///
/// # Returns
///
/// A 64-bit bitmask representing the rotated board state.
///
/// # Example
///
/// ```
/// use temp_reversi_core::utils::rotate_mask_90_clockwise;
///
/// let mask = 0x00000000000000FF; // Row 0 (all bits in the first row are set)
/// let rotated = rotate_mask_90_clockwise(mask);
/// assert_eq!(rotated, 0x8080808080808080); // Column 7 (rotated to the last column)
/// ```
pub fn rotate_mask_90_clockwise(mask: u64) -> u64 {
    let mut rotated = 0u64;

    for row in 0..8 {
        for col in 0..8 {
            // Calculate the bit position in the original mask
            let original_bit = 1 << (row * 8 + col);

            // If the bit is set, calculate its new position in the rotated board
            if mask & original_bit != 0 {
                let rotated_bit = 1 << ((7 - row) + col * 8);
                rotated |= rotated_bit;
            }
        }
    }

    rotated
}
