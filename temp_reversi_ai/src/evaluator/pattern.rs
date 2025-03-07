use std::sync::Arc;

use super::EvaluationFunction;
use crate::{
    learning::{extract_features, Model},
    patterns::PatternGroup,
    utils::Feature,
};
use temp_reversi_core::{Board, Player};

/// Evaluates the board based on multiple pattern groups and their scores.
#[derive(Debug, Clone)]
pub struct PatternEvaluator {
    /// Collection of pattern groups.
    pub groups: Arc<Vec<PatternGroup>>,
    pub model: Arc<Model>,
}

impl PatternEvaluator {
    /// Creates a `PatternEvaluator` with a predefined list of pattern groups.
    ///
    /// # Arguments
    /// * `groups` - A vector of `PatternGroup` instances to be managed by the evaluator.
    ///
    /// # Returns
    /// A `PatternEvaluator` initialized with the provided pattern groups.
    pub fn new(groups: Vec<PatternGroup>, model: Model) -> Self {
        Self {
            groups: Arc::new(groups),
            model: Arc::new(model),
        }
    }
}

impl EvaluationFunction for PatternEvaluator {
    fn evaluate(&self, board: &impl Board, player: Player) -> i32 {
        let vector = extract_features(board, &self.groups);

        // phase[0] = 1手進めた盤面
        // phase[1] = 2手進めた盤面
        // ...
        let total_stones = board.count_stones().0 + board.count_stones().1;
        let phase = (total_stones - 5).max(0);

        let feature = Feature { phase, vector };

        let value = self.model.predict(&[feature]);

        if player == Player::White {
            -value[0] as i32
        } else {
            value[0] as i32
        }
    }
}
