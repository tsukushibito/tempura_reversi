use temp_reversi_core::Bitboard;

use super::{coordinate_to_feature::C2F_LISTS, patterns::PATTERNS};

#[derive(Debug, Clone)]
pub struct Feature {
    pub vector: [u32; PATTERNS.len()],
    pub phase: u8,
}

impl Default for Feature {
    fn default() -> Self {
        Feature {
            vector: [0; PATTERNS.len()],
            phase: 0,
        }
    }
}

pub fn extract_feature(board: &Bitboard) -> Feature {
    let mut feature = Feature {
        vector: [0; PATTERNS.len()],
        phase: 0,
    };

    let (black, white) = board.count_stones();
    feature.phase = (black + white) as u8;

    let squares = squares_from_bitboard(board);
    for i in 0..64 {
        C2F_LISTS[i].iter().for_each(|&c2f| {
            let s = squares[i] as u16;
            let f = c2f.trit_place_value * s;
            feature.vector[c2f.pattern_index as usize] += f as u32;
        });
    }

    feature
}

fn squares_from_bitboard(bitboard: &Bitboard) -> [u8; 64] {
    let mut squares = [0; 64];

    let (black, white) = bitboard.bits();

    for i in 0..64 {
        if (black >> i) & 1 == 1 {
            squares[i] = 0;
        } else if (white >> i) & 1 == 1 {
            squares[i] = 1;
        } else {
            squares[i] = 2;
        }
    }

    squares
}

#[cfg(test)]
mod tests {
    use temp_reversi_core::Position;

    use crate::coordinate::*;

    use super::*;

    #[test]
    fn test_squares_from_bitboard() {
        let bitboard = Bitboard::default();
        let squares = squares_from_bitboard(&bitboard);

        for i in 0..64 {
            if i == D4 || i == E5 {
                assert_eq![squares[i as usize], 1]; // White square
            } else if i == D5 || i == E4 {
                assert_eq![squares[i as usize], 0]; // Black square
            } else {
                assert_eq![squares[i as usize], 2]; // Empty square
            }
        }
    }

    #[test]
    fn test_extract_feature() {
        let black = Position::A1 | Position::B1 | Position::C1;
        let white = Position::A2 | Position::B2 | Position::C2;
        let bitboard = Bitboard::new(black, white);

        let feature = extract_feature(&bitboard);

        // 3^0 * 1 + 3^1 * 1 + 3^2 * 1 +
        // 3^3 * 2 + 3^4 * 2 + 3^5 * 2 +
        // 3^6 * 2 + 3^7 * 2 =
        assert_eq!(feature.vector[0], 6547);

        // 3^0 * 0 + 3^1 * 0 + 3^2 * 0 +
        // 3^3 * 1 + 3^4 * 1 + 3^5 * 1 +
        // 3^6 * 2 + 3^7 * 2 + 3^8 * 2 = 19305
        assert_eq![feature.vector[44], 19305];
    }
}
