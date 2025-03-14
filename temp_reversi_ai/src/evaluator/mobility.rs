use temp_reversi_core::{Board, Player};

use super::EvaluationFunction;

/// Mobility evaluator that considers the number of valid moves as the score.
pub struct MobilityEvaluator;

impl<B: Board> EvaluationFunction<B> for MobilityEvaluator {
    fn evaluate(&self, board: &B, player: Player) -> i32 {
        // Calculate mobility for the current player and opponent
        let player_mobility = board.valid_moves(player).len() as i32;
        let opponent_mobility = board.valid_moves(player.opponent()).len() as i32;

        // Mobility score is the difference between the player's and the opponent's mobility
        player_mobility - opponent_mobility
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use temp_reversi_core::{Bitboard, Player};

    #[test]
    fn test_mobility_evaluation() {
        let board = Bitboard::default(); // Default board with initial setup
        let evaluator = MobilityEvaluator;

        // Test Black's perspective
        let black_score = evaluator.evaluate(&board, Player::Black);
        assert!(
            black_score >= 0,
            "Black's mobility score should be non-negative."
        );

        // Test White's perspective
        let white_score = evaluator.evaluate(&board, Player::White);
        assert!(
            white_score >= 0,
            "White's mobility score should be non-negative."
        );

        // Ensure the score is symmetric
        assert_eq!(
            black_score,
            -evaluator.evaluate(&board, Player::White),
            "Black's score should be the negative of White's score."
        );
    }
}
