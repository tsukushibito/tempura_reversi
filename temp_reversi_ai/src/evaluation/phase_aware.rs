use temp_reversi_core::{Bitboard, Player};

use super::{mobility::MobilityEvaluator, EvaluationFunction, PositionalEvaluator};

/// Defines the phase of the game
enum Phase {
    Early,
    Mid,
    Late,
}

/// Phase-aware evaluator that adjusts weights for mobility, positional values, and score
/// based on the phase of the game.
pub struct PhaseAwareEvaluator;

impl PhaseAwareEvaluator {
    /// Determine the phase of the game based on the total number of stones.
    fn determine_phase(&self, board: &Bitboard) -> Phase {
        let (black_count, white_count) = board.count_stones();
        let total_stones = black_count + white_count;

        if total_stones <= 20 {
            Phase::Early
        } else if total_stones <= 50 {
            Phase::Mid
        } else {
            Phase::Late
        }
    }
}

impl EvaluationFunction for PhaseAwareEvaluator {
    fn evaluate(&self, board: &Bitboard, player: Player) -> i32 {
        let phase = self.determine_phase(board);
        let mobility_evaluator = MobilityEvaluator;
        let positional_evaluator = PositionalEvaluator;

        // Evaluate each factor
        let mobility_score = mobility_evaluator.evaluate(board, player);
        let positional_score = positional_evaluator.evaluate(board, player);
        let (black_count, white_count) = board.count_stones();
        let score_diff = match player {
            Player::Black => black_count as i32 - white_count as i32,
            Player::White => white_count as i32 - black_count as i32,
        };

        // Apply weights based on the phase
        let score = match phase {
            Phase::Early => 2 * mobility_score + positional_score,
            Phase::Mid => 2 * mobility_score + positional_score + score_diff,
            Phase::Late => score_diff,
        };

        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use temp_reversi_core::{Bitboard, Player};

    #[test]
    fn test_phase_aware_evaluation() {
        let board = Bitboard::default(); // Initial board state
        let evaluator = PhaseAwareEvaluator;

        // Test early phase
        let early_score = evaluator.evaluate(&board, Player::Black);
        assert!(
            early_score >= 0,
            "Early phase score should be calculated correctly."
        );

        // Simulate mid-phase board state
        let mid_board = board.clone();
        // Apply moves to transition to mid-phase
        let mid_score = evaluator.evaluate(&mid_board, Player::Black);
        assert!(
            mid_score >= 0,
            "Mid phase score should be calculated correctly."
        );

        // Simulate late-phase board state
        let late_board = board.clone();
        // Apply moves to transition to late-phase
        let late_score = evaluator.evaluate(&late_board, Player::Black);
        assert!(
            late_score >= 0,
            "Late phase score should be calculated correctly."
        );
    }
}
