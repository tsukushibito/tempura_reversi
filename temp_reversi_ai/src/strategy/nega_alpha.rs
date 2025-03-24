use temp_game_ai::searcher::{NegaAlpha, Searcher};
use temp_reversi_core::{Bitboard, Player, Position};

use crate::{evaluator::TempuraEvaluator, ReversiState};

use super::Strategy;

/// The Negamax strategy with alpha-beta pruning.
#[derive(Clone, Debug)]
pub struct NegaAlphaStrategy {
    pub nega_alpha: NegaAlpha<ReversiState, TempuraEvaluator>,
    pub max_depth: usize,
}

impl NegaAlphaStrategy {
    pub fn new(model_path: &str, depth: usize) -> Self {
        let evaluator = TempuraEvaluator::new(model_path);
        let nega_alpha = NegaAlpha::new(evaluator);
        Self {
            nega_alpha,
            max_depth: depth,
        }
    }
}

impl Strategy for NegaAlphaStrategy {
    fn select_move(&mut self, board: &Bitboard, player: Player) -> Position {
        let mut state = ReversiState::new(*board, player);

        self.nega_alpha
            .search(&mut state, self.max_depth)
            .expect("No moves available.")
            .0
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
