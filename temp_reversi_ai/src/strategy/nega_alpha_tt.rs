use temp_game_ai::search::NegaAlphaTT;
use temp_reversi_core::{Bitboard, Player};

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
    fn evaluate_and_decide(
        &mut self,
        board: &Bitboard,
        player: Player,
    ) -> Option<temp_reversi_core::Position> {
        let root = ReversiState {
            board: *board,
            player,
        };

        let best_move = self.nega_alpha_tt.search_best_move(&root, self.max_depth);
        Some(best_move)
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        todo!()
    }
}
