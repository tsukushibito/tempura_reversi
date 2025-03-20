use temp_game_ai::searcher::{NegaAlphaTT, Searcher};
use temp_reversi_core::{Bitboard, Player, Position};

use crate::evaluator::{PhaseAwareEvaluator, ReversiState, TempuraEvaluator};

use super::Strategy;

#[derive(Clone, Debug)]
pub struct NegaAlphaTTStrategy {
    pub nega_alpha_tt: NegaAlphaTT<ReversiState, TempuraEvaluator, PhaseAwareEvaluator>,
    max_depth: usize,
}

impl NegaAlphaTTStrategy {
    pub fn new(evaluator: TempuraEvaluator, max_depth: usize) -> Self {
        let order_evaluator = PhaseAwareEvaluator::default();
        let nega_alpha_tt = NegaAlphaTT::new(evaluator, order_evaluator);
        Self {
            nega_alpha_tt,
            max_depth,
        }
    }
}

impl Strategy for NegaAlphaTTStrategy {
    fn select_move(&mut self, board: &Bitboard, player: Player) -> Option<Position> {
        let root = ReversiState {
            board: *board,
            player,
        };

        if let Some(best_move) = self.nega_alpha_tt.search(&root, self.max_depth) {
            Some(best_move.0)
        } else {
            None
        }
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
