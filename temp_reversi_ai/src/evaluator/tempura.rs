use crate::{learning::Model, patterns::get_predefined_patterns};

use super::pattern::PatternEvaluator;
use super::phase_aware::PhaseAwareEvaluator;
use super::EvaluationFunction;
use temp_reversi_core::{Board, Player};

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
            println!("Model loaded from: {}", model_path);
            let pattern_evaluator = PatternEvaluator::new(get_predefined_patterns(), model);
            Self {
                phase_aware: PhaseAwareEvaluator::default(),
                pattern: Some(pattern_evaluator),
            }
        } else {
            println!("Failed to load model from: {}", model_path);
            Self {
                phase_aware: PhaseAwareEvaluator::default(),
                pattern: None,
            }
        }
    }
}

impl<B: Board> EvaluationFunction<B> for TempuraEvaluator {
    fn evaluate(&self, board: &B, player: Player) -> i32 {
        if let Some(pattern) = &self.pattern {
            pattern.evaluate(board, player)
        } else {
            self.phase_aware.evaluate(board, player)
        }
    }
}
