use temp_reversi_core::Bitboard;

use super::patterns::PATTERNS;

pub struct Feature {
    pub indices: [u16; PATTERNS.len()],
}

impl Feature {
    pub fn new(board: &Bitboard) -> Feature {
        Feature {
            indices: [0; PATTERNS.len()],
        }
    }

    fn extract_feature(board: &Bitboard, feature: &mut Feature) {
        for i in 0..64 {
            todo!("");
        }
    }
}
