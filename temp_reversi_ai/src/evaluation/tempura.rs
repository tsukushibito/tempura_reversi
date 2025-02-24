use crate::{learning::Model, patterns::get_predefined_patterns};

use super::pattern::PatternEvaluator;
use super::phase_aware::PhaseAwareEvaluator;
use super::EvaluationFunction;
use temp_reversi_core::{Bitboard, Player};

#[derive(Debug, Clone)]
pub struct TempuraEvaluator {
    phase_aware: PhaseAwareEvaluator,
    pattern: Option<PatternEvaluator>,
}

impl TempuraEvaluator {
    // Create a new TempuraEvaluator.
    // Pass in the PatternEvaluator since configuration data may be needed.
    pub fn new(model_path: &str) -> Self {
        if let Ok(model) = Model::load(model_path) {
            let pattern_evaluator = PatternEvaluator::new(get_predefined_patterns(), model);
            Self {
                phase_aware: PhaseAwareEvaluator,
                pattern: Some(pattern_evaluator),
            }
        } else {
            Self {
                phase_aware: PhaseAwareEvaluator,
                pattern: None,
            }
        }
    }
}

impl EvaluationFunction for TempuraEvaluator {
    fn evaluate(&self, board: &Bitboard, player: Player) -> i32 {
        if let Some(pattern) = &self.pattern {
            // Determine phase by counting stones.
            let (black, white) = board.count_stones();
            let total = black + white;
            if total <= 48 {
                // Early phase: use PhaseAwareEvaluator.
                self.phase_aware.evaluate(board, player)
            } else {
                // Mid and Late phases: use PatternEvaluator.
                pattern.evaluate(board, player)
            }
        } else {
            // Fallback to PhaseAwareEvaluator.
            self.phase_aware.evaluate(board, player)
        }
    }
}
