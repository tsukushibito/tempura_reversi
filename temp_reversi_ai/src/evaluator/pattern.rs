use crate::{
    learning::{extract_features, Model},
    patterns::{get_predefined_patterns, PatternGroup},
    utils::Feature,
};
use temp_game_ai::Evaluator;
use temp_reversi_core::{Bitboard, Player};

use crate::ReversiState;

/// Evaluates the board based on multiple pattern groups and their scores.
#[derive(Debug, Clone)]
pub struct PatternEvaluator {
    /// Collection of pattern groups.
    pub pattern_groups: Vec<PatternGroup>,
    pub model: Model,
    pub use_impl2: bool,
}

impl PatternEvaluator {
    /// Creates a `PatternEvaluator` with a predefined list of pattern groups.
    ///
    /// # Arguments
    /// * `groups` - A vector of `PatternGroup` instances to be managed by the evaluator.
    ///
    /// # Returns
    /// A `PatternEvaluator` initialized with the provided pattern groups.
    pub fn new(model: Model) -> Self {
        let mut pattern_groups = get_predefined_patterns();

        for (phase, weights) in model.weights.iter().enumerate() {
            let mut index_offset = 0;
            for pattern_group in &mut pattern_groups {
                let state_scores = &mut pattern_group.state_scores[phase];
                state_scores
                    .iter_mut()
                    .enumerate()
                    .for_each(|(i, score)| *score = weights[index_offset + i]);
                index_offset += state_scores.len();
            }
        }

        Self {
            pattern_groups,
            model,
            use_impl2: true,
        }
    }

    fn evaluate_impl2(&mut self, board: &Bitboard, player: Player) -> i32 {
        // phase[0] = 1手進めた盤面
        // phase[1] = 2手進めた盤面
        // ...
        let total_stones = (board.count_stones().0 + board.count_stones().1) as i32;
        let phase = (total_stones - 5).max(0) as usize;

        let value: f32 = self
            .pattern_groups
            .iter_mut()
            .map(|group| group.evaluate_score(board, phase))
            .sum();

        if player == Player::Black {
            value as i32
        } else {
            -value as i32
        }
    }

    fn evaluate_impl1(&mut self, board: &Bitboard, player: Player) -> i32 {
        let vector = extract_features(board, &self.pattern_groups);

        // phase[0] = 1手進めた盤面
        // phase[1] = 2手進めた盤面
        // ...
        let total_stones = (board.count_stones().0 + board.count_stones().1) as i32;
        let phase = (total_stones - 5).max(0) as usize;

        let feature = Feature { phase, vector };

        let value = self.model.predict(&[feature]);

        if player == Player::White {
            -value[0] as i32
        } else {
            value[0] as i32
        }
    }
}

impl Evaluator<ReversiState> for PatternEvaluator {
    fn evaluate(&mut self, state: &ReversiState) -> i32 {
        if self.use_impl2 {
            self.evaluate_impl2(&state.board, state.player)
        } else {
            self.evaluate_impl1(&state.board, state.player)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use temp_reversi_core::Bitboard;

    #[test]
    fn test_pattern_evaluator() {
        let model = Model::load("../gen0/models/temp_model.bin").unwrap();
        let mut evaluator = PatternEvaluator::new(model);

        let mut board = Bitboard::default();
        for _i in 0..4 {
            let valid_moves = board.valid_moves(Player::Black);
            let mov = valid_moves.first().unwrap();
            let _ = board.apply_move(*mov, Player::Black);
            let valid_moves = board.valid_moves(Player::White);
            let mov = valid_moves.first().unwrap();
            let _ = board.apply_move(*mov, Player::White);
        }

        println!("{}", board);
        let state = ReversiState::new(board, Player::Black);
        evaluator.use_impl2 = true;
        let score1 = evaluator.evaluate(&state);
        evaluator.use_impl2 = false;
        let score2 = evaluator.evaluate(&state);

        assert_eq!(score1, score2);

        evaluator.use_impl2 = true;
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _score = evaluator.evaluate(&state);
        }
        let elapsed = start.elapsed();
        println!("evaluate_impl2 elapsed: {:?}", elapsed);

        evaluator.use_impl2 = false;
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _score = evaluator.evaluate(&state);
        }
        let elapsed = start.elapsed();
        println!("evaluate2_impl1 elapsed: {:?}", elapsed);
    }
}
