/// Rotates a 64-bit bitmask representing an 8x8 board 90 degrees clockwise.
///
/// # Arguments
/// * `mask` - A 64-bit integer representing the bitmask of the board.
///
/// # Returns
/// A new 64-bit integer where the bits are rotated 90 degrees clockwise.
pub fn rotate_mask_90_cw(mask: u64) -> u64 {
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

/// Rotates a 64-bit bitmask representing an 8x8 board 90 degrees counterclockwise.
///
/// # Arguments
/// * `mask` - A 64-bit integer representing the bitmask of the board.
///
/// # Returns
/// A new 64-bit integer where the bits are rotated 90 degrees counterclockwise.
pub fn rotate_mask_90_ccw(mask: u64) -> u64 {
    let mut rotated = 0u64;

    for row in 0..8 {
        for col in 0..8 {
            // Calculate the bit position in the original mask
            let original_bit = 1 << (row * 8 + col);

            // If the bit is set, calculate its new position in the rotated board
            if mask & original_bit != 0 {
                let rotated_bit = 1 << ((7 - col) * 8 + row);
                rotated |= rotated_bit;
            }
        }
    }

    rotated
}

/// Rotates a 64-bit bitmask representing an 8x8 board 180 degrees.
///
/// # Arguments
/// * `mask` - A 64-bit integer representing the bitmask of the board.
///
/// # Returns
/// A new 64-bit integer where the bits are rotated 180 degrees.
pub fn rotate_mask_180(mask: u64) -> u64 {
    rotate_mask_90_cw(rotate_mask_90_cw(mask))
}

/// Rotates a 64-bit bitmask representing an 8x8 board 270 degrees clockwise.
///
/// # Arguments
/// * `mask` - A 64-bit integer representing the bitmask of the board.
///
/// # Returns
/// A new 64-bit integer where the bits are rotated 270 degrees clockwise.
pub fn rotate_mask_270_cw(mask: u64) -> u64 {
    rotate_mask_90_ccw(mask) // 270 degrees clockwise is the same as 90 degrees counterclockwise
}

/// Rotates a 64-bit bitmask representing an 8x8 board 270 degrees counterclockwise.
///
/// # Arguments
/// * `mask` - A 64-bit integer representing the bitmask of the board.
///
/// # Returns
/// A new 64-bit integer where the bits are rotated 270 degrees counterclockwise.
pub fn rotate_mask_270_ccw(mask: u64) -> u64 {
    rotate_mask_90_cw(mask) // 270 degrees counterclockwise is the same as 90 degrees clockwise
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_mask_90_cw() {
        // Single bit at (row=0, col=7) -> Moves to (row=7, col=7)
        assert_eq!(rotate_mask_90_cw(0x0000000000000080), 0x8000000000000000);

        // Bottom row (row=7) -> Becomes leftmost column (col=0)
        assert_eq!(rotate_mask_90_cw(0xFF00000000000000), 0x0101010101010101);

        // Center vertical line -> Becomes center horizontal line
        assert_eq!(rotate_mask_90_cw(0x1010101010101010), 0x000000FF00000000);
    }

    #[test]
    fn test_rotate_mask_90_ccw() {
        // Single bit at (row=0, col=7) -> Moves to (row=0, col=0)
        assert_eq!(rotate_mask_90_ccw(0x0000000000000080), 0x0000000000000001);

        // Bottom row (row=7) -> Becomes rightmost column (col=7)
        assert_eq!(rotate_mask_90_ccw(0xFF00000000000000), 0x8080808080808080);

        // Center vertical line -> Becomes center horizontal line (opposite direction)
        assert_eq!(rotate_mask_90_ccw(0x1010101010101010), 0x00000000FF000000);
    }

    #[test]
    fn test_rotate_mask_180() {
        // Single bit at (row=0, col=7) -> Moves to (row=7, col=0)
        assert_eq!(rotate_mask_180(0x0000000000000080), 0x0100000000000000);

        // Bottom row remains at bottom, reversed
        assert_eq!(rotate_mask_180(0x00000000000000FF), 0xFF00000000000000);

        // Vertical center line
        assert_eq!(rotate_mask_180(0x1010101010101010), 0x0808080808080808);

        // Diagonal reflection
        assert_eq!(rotate_mask_180(0x8040201008040201), 0x8040201008040201);
    }

    #[test]
    fn test_rotate_mask_270_cw() {
        // Single bit at (row=0, col=7) -> Moves to (row=0, col=0)
        assert_eq!(rotate_mask_270_cw(0x0000000000000080), 0x0000000000000001);

        // Bottom row (row=7) -> Becomes rightmost column (col=7)
        assert_eq!(rotate_mask_270_cw(0xFF00000000000000), 0x8080808080808080);

        // Center vertical line -> Becomes center horizontal line (opposite direction)
        assert_eq!(rotate_mask_270_cw(0x1010101010101010), 0x00000000FF000000);
    }

    #[test]
    fn test_rotate_mask_270_ccw() {
        // Single bit at (row=0, col=7) -> Moves to (row=7, col=7)
        assert_eq!(rotate_mask_270_ccw(0x0000000000000080), 0x8000000000000000);

        // Bottom row (row=7) -> Becomes leftmost column (col=0)
        assert_eq!(rotate_mask_270_ccw(0xFF00000000000000), 0x0101010101010101);

        // Center vertical line -> Becomes center horizontal line
        assert_eq!(rotate_mask_270_ccw(0x1010101010101010), 0x000000FF00000000);
    }
}
