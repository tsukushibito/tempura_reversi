use crate::learning::Model;

use super::pattern::PatternEvaluator;
use super::phase_aware::PhaseAwareEvaluator;
use super::Evaluator;
use temp_reversi_core::{Bitboard, Player};

#[derive(Debug, Clone)]
pub struct TempuraEvaluator {
    pub phase_aware: PhaseAwareEvaluator,
    pub pattern: Option<PatternEvaluator>,
}

impl TempuraEvaluator {
    // Create a new TempuraEvaluator.
    // Pass in the PatternEvaluator since configuration data may be needed.
    pub fn new(model_path: &str) -> Self {
        if let Ok(model) = Model::load(model_path) {
            println!("Model loaded from: {}", model_path);
            let pattern_evaluator = PatternEvaluator::new(model);
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

impl Evaluator for TempuraEvaluator {
    fn evaluate(&mut self, board: &Bitboard, player: Player) -> i32 {
        if let Some(pattern) = &mut self.pattern {
            pattern.evaluate(board, player)
        } else {
            self.phase_aware.evaluate(board, player)
        }
    }
}
