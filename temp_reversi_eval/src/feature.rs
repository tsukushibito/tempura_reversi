use temp_reversi_core::Bitboard;

use crate::patterns::get_symmetric_pattern_indices;

use super::{coordinate_to_feature::C2F_LISTS, patterns::PATTERNS};

pub const PHASE_MAX: u8 = 65;

/// The `Feature` struct represents the feature vector for a given board state.
/// It contains the indices for each pattern and the phase of the game.
#[derive(Debug, Clone)]
pub struct Feature {
    /// The indices for each pattern in the feature vector.
    /// Each index corresponds to a specific pattern in the `PATTERNS` array.
    pub indices: [u16; PATTERNS.len()],

    /// The phase of the game, which is the sum of black and white stones on the board.
    /// This is used to determine the current state of the game.
    pub phase: u8,
}

impl Default for Feature {
    fn default() -> Self {
        Feature {
            indices: [0; PATTERNS.len()],
            phase: 0,
        }
    }
}

/// Extracts the feature vector from the given `Bitboard` representation of the game state.
/// The feature vector is computed based on the positions of black and white stones on the board.
pub fn extract_feature(board: &Bitboard) -> Feature {
    let mut feature = Feature {
        indices: [0; PATTERNS.len()],
        phase: 0,
    };

    let (black, white) = board.count_stones();
    feature.phase = (black + white) as u8;

    let squares = squares_from_bitboard(board);
    for i in 0..64 {
        C2F_LISTS[i].iter().for_each(|&c2f| {
            let s = squares[i] as u16;
            let f = c2f.trit_place_value * s;
            feature.indices[c2f.pattern_index as usize] += f;
        });
    }

    feature
}

pub fn canonicalize_pattern_feature(pattern_index: usize, feature_index: u16) -> u16 {
    let pattern = &PATTERNS[pattern_index];
    let symmetric_pattern_indices = get_symmetric_pattern_indices(pattern_index);

    let mut symmetric_feature = 0;
    for i in 0..pattern.len() {
        let symmetric_index = symmetric_pattern_indices[i] as u32;
        symmetric_feature += (feature_index / 3u16.pow(symmetric_index)) % 3 * 3u16.pow(i as u32);
    }

    feature_index.min(symmetric_feature)
}

pub fn canonicalize_feature(feature: &Feature) -> Feature {
    let mut canonical_feature = feature.clone();
    for (i, &index) in feature.indices.iter().enumerate() {
        canonical_feature.indices[i] = canonicalize_pattern_feature(i, index);
    }
    canonical_feature
}

/// Converts the given `Bitboard` into a square representation.
/// Each square is represented as a value in the range [0, 2], where:
pub(super) fn squares_from_bitboard(bitboard: &Bitboard) -> [u8; 64] {
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
        // 3^6 * 2 + 3^7 * 2 = 6547
        assert_eq!(feature.indices[0], 6547);

        // 3^0 * 0 + 3^1 * 0 + 3^2 * 0 +
        // 3^3 * 1 + 3^4 * 1 + 3^5 * 1 +
        // 3^6 * 2 + 3^7 * 2 + 3^8 * 2 = 19305
        assert_eq![feature.indices[44], 19305];
    }

    #[test]
    fn test_canonicalize_feature() {
        let black = Position::A1 | Position::B1 | Position::C1;
        let white = Position::A2 | Position::B2 | Position::C2;
        let bitboard = Bitboard::new(black, white);

        let feature = extract_feature(&bitboard);
        let canonical_feature = canonicalize_feature(&feature);

        // 3^0 * 2 + 3^1 * 2 + 3^2 * 2 +
        // 3^3 * 2 + 3^4 * 2 + 3^5 * 1 +
        // 3^6 * 1 + 3^7 * 1 = 3401
        // Symmetric pattern indices are used here
        // The feature vector is normalized by selecting the smaller value between the original and symmetric feature
        assert_eq!(canonical_feature.indices[0], 3401);

        // 3^0 * 0 + 3^1 * 1 + 3^2 * 2 +
        // 3^3 * 0 + 3^4 * 1 + 3^5 * 2 +
        // 3^6 * 0 + 3^7 * 1 + 3^8 * 2 = 15897
        assert_eq![canonical_feature.indices[44], 15897];

        // 3^0 * 0 + 3^1 * 1 + 3^2 * 2 +
        // 3^3 * 2 + 3^4 * 2 + 3^5 * 2 +
        // 3^6 * 2 + 3^7 * 2 + 3^8 * 1 +
        // 3^9 * 0 =
        assert_eq![canonical_feature.indices[12], 13116];
    }
}
