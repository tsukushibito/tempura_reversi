use temp_reversi_core::{Bitboard, Player};

use super::Evaluator;

/// Positional evaluator that considers board position values.
pub struct PositionalEvaluator;

impl Evaluator for PositionalEvaluator {
    fn evaluate(&mut self, board: &Bitboard, player: Player) -> i32 {
        // Positional values for the board (example values for demonstration)
        let positional_values: [i32; 64] = [
            100, -20, 10, 5, 5, 10, -20, 100, // Row 1
            -20, -50, -2, -2, -2, -2, -50, -20, // Row 2
            10, -2, 3, 2, 2, 3, -2, 10, // Row 3
            5, -2, 2, 0, 0, 2, -2, 5, // Row 4
            5, -2, 2, 0, 0, 2, -2, 5, // Row 5
            10, -2, 3, 2, 2, 3, -2, 10, // Row 6
            -20, -50, -2, -2, -2, -2, -50, -20, // Row 7
            100, -20, 10, 5, 5, 10, -20, 100, // Row 8
        ];

        let (black_bits, white_bits) = board.bits();
        let mut score = 0;

        // Calculate score using bitboard representation
        for i in 0..64 {
            let mask = 1u64 << i;
            if black_bits & mask != 0 {
                score += positional_values[i];
            } else if white_bits & mask != 0 {
                score -= positional_values[i];
            }
        }

        // Adjust score based on the player perspective
        match player {
            Player::Black => score,
            Player::White => -score,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use temp_reversi_core::{Bitboard, Player};

    #[test]
    fn test_positional_evaluation() {
        let board = Bitboard::default(); // Default board with initial setup
        let mut evaluator = PositionalEvaluator;

        // Test Black's perspective
        let black_score = evaluator.evaluate(&board, Player::Black);
        assert_eq!(
            black_score, 0,
            "Black should have a score of 0 on the default board."
        );

        // Test White's perspective
        let white_score = evaluator.evaluate(&board, Player::White);
        assert_eq!(
            white_score, 0,
            "White should have a score of 0 on the default board."
        );
    }
}
