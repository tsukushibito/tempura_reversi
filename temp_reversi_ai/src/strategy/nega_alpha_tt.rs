use temp_game_ai::{
    searcher::{NegaAlphaTT, Searcher},
    Evaluator,
};
use temp_reversi_core::{Bitboard, Player, Position};

use crate::ReversiState;

use super::Strategy;

#[derive(Clone, Debug)]
pub struct NegaAlphaTTStrategy<E, O>
where
    E: Evaluator<ReversiState>,
    O: Evaluator<ReversiState>,
{
    pub nega_alpha_tt: NegaAlphaTT<ReversiState, E, O>,
    max_depth: usize,
}

impl<E, O> NegaAlphaTTStrategy<E, O>
where
    E: Evaluator<ReversiState>,
    O: Evaluator<ReversiState>,
{
    pub fn new(evaluator: E, order_evaluator: O, max_depth: usize) -> Self {
        let nega_alpha_tt = NegaAlphaTT::new(evaluator, order_evaluator);
        Self {
            nega_alpha_tt,
            max_depth,
        }
    }
}

impl<E, O> Strategy for NegaAlphaTTStrategy<E, O>
where
    E: Evaluator<ReversiState> + Clone + 'static,
    O: Evaluator<ReversiState> + Clone + 'static,
{
    fn select_move(&mut self, board: &Bitboard, player: Player) -> Position {
        let mut state = ReversiState::new(*board, player);

        self.nega_alpha_tt
            .search(&mut state, self.max_depth)
            .expect("No moves available.")
            .0
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
