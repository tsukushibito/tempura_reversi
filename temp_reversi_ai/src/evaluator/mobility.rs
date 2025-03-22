use temp_game_ai::Evaluator;

use crate::ReversiState;

/// Mobility evaluator that considers the number of valid moves as the score.
pub struct MobilityEvaluator;

impl Evaluator<ReversiState> for MobilityEvaluator {
    fn evaluate(&mut self, state: &ReversiState) -> i32 {
        // Calculate mobility for the current player and opponent
        let player_mobility = state.board.valid_moves(state.player).len() as i32;
        let opponent_mobility = state.board.valid_moves(state.player.opponent()).len() as i32;

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
        let mut evaluator = MobilityEvaluator;

        // Test Black's perspective
        let black_score = evaluator.evaluate(&ReversiState {
            board,
            player: Player::Black,
        });
        assert!(
            black_score >= 0,
            "Black's mobility score should be non-negative."
        );

        // Test White's perspective
        let white_score = evaluator.evaluate(&ReversiState {
            board,
            player: Player::White,
        });
        assert!(
            white_score >= 0,
            "White's mobility score should be non-negative."
        );

        // Ensure the score is symmetric
        assert_eq!(
            black_score,
            -evaluator.evaluate(&ReversiState {
                board,
                player: Player::White,
            }),
            "Black's score should be the negative of White's score."
        );
    }
}
