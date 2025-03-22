use crate::learning::Model;
use crate::ReversiState;

use super::pattern::PatternEvaluator;
use super::phase_aware::PhaseAwareEvaluator;

use temp_game_ai::Evaluator;

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

impl Evaluator<ReversiState> for TempuraEvaluator {
    fn evaluate(&mut self, state: &ReversiState) -> i32 {
        if let Some(pattern) = &mut self.pattern {
            pattern.evaluate(state)
        } else {
            self.phase_aware.evaluate(state)
        }
    }
}
