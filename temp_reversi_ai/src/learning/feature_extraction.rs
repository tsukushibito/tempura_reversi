use temp_reversi_core::Bitboard;

use crate::{evaluation::PatternEvaluator, patterns::get_predefined_patterns};

pub fn extract_features(board: &Bitboard) -> Vec<f64> {
    let mut features = Vec::new();

    let evaluator = PatternEvaluator::new(get_predefined_patterns());
    let (black_mask, white_mask) = board.bits();

    for group in &evaluator.groups {
        for pattern in &group.patterns {
            let masked_black = black_mask & pattern.mask;
            let masked_white = white_mask & pattern.mask;

            match pattern.key_to_index.get(&(masked_black, masked_white)) {
                Some(&state_index) => features.push(state_index as f64),
                None => panic!(
                    "Unexpected state in pattern evaluation: masked_black = {:#b}, masked_white = {:#b}",
                    masked_black, masked_white
                ),
            }
        }
    }

    features
}
#[cfg(test)]
mod tests {
    use super::*;

    /// `panic!` が発生しないことを確認するテスト
    #[test]
    fn test_extract_features_no_panic() {
        let board = Bitboard::default();
        let _ = extract_features(&board); // `panic!` しないことを期待
    }
}
