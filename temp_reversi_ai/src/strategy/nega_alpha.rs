use temp_game_ai::searcher::{NegaAlpha, Searcher};
use temp_reversi_core::{Bitboard, Player, Position};

use crate::evaluator::{ReversiState, TempuraEvaluator};

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
    fn select_move(&mut self, board: &Bitboard, player: Player) -> Option<Position> {
        let root = ReversiState {
            board: *board,
            player,
        };

        if let Some(best_move) = self.nega_alpha.search(&root, self.max_depth) {
            Some(best_move.0)
        } else {
            None
        }
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
