use temp_reversi_ai::ReversiState;

use crate::{
    feature::{extract_feature, Feature},
    feature_offsets::FEATURE_OFFSETS,
    model::Model,
    patterns::PATTERNS,
};

pub struct Evaluator {
    model: Model,
    features: [Feature; 64],
}

impl Evaluator {
    pub fn new(model: Model) -> Self {
        let mut features: [Feature; 64] = std::array::from_fn(|_| Feature::default());
        Self { model, features }
    }

    pub fn evaluate(&mut self, state: &ReversiState) -> f32 {
        let (black, white) = state.board.count_stones();
        let phase = (black + white).max(0) as usize;

        // temporary
        // TODO: Use the previous phase to calculate the feature
        self.features[phase] = extract_feature(&state.board);

        let feature = &self.features[phase];
        let weights = &self.model.weights[phase];

        let mut value = 0.0;
        for i in 0..feature.vector.len() {
            let index = feature.vector[i] + FEATURE_OFFSETS[i];
            value += weights[index as usize];
        }

        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use temp_reversi_core::Bitboard;

    #[test]
    fn test_evaluator() {
        todo!()
    }
}
