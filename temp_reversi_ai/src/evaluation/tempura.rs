use super::pattern::PatternEvaluator;
use super::phase_aware::PhaseAwareEvaluator;
use super::EvaluationFunction;
use temp_reversi_core::{Bitboard, Player};

#[derive(Debug, Clone)]
pub struct TempuraEvaluator {
    phase_aware: PhaseAwareEvaluator,
    pattern: PatternEvaluator,
}

impl TempuraEvaluator {
    // Create a new TempuraEvaluator.
    // Pass in the PatternEvaluator since configuration data may be needed.
    pub fn new(pattern: PatternEvaluator) -> Self {
        Self {
            phase_aware: PhaseAwareEvaluator,
            pattern,
        }
    }
}

impl EvaluationFunction for TempuraEvaluator {
    fn evaluate(&self, board: &Bitboard, player: Player) -> i32 {
        // Determine phase by counting stones.
        let (black, white) = board.count_stones();
        let total = black + white;
        if total <= 49 {
            // Early phase: use PhaseAwareEvaluator.
            self.phase_aware.evaluate(board, player)
        } else {
            // Mid and Late phases: use PatternEvaluator.
            self.pattern.evaluate(board, player)
        }
    }
}
