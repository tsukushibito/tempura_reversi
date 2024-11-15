use crate::{player::Player, search::Negaalpha, Board, GameState};
use reversi_core::{Color, Position};

pub struct AIPlayer<B: Board> {
    searcher: Negaalpha<B>,
    color: Color,
    // 必要に応じて他のフィールドを追加
}

impl<B: Board + Clone + std::hash::Hash + Eq> AIPlayer<B> {
    pub fn new(evaluate_fn: fn(&GameState<B>, Color) -> i32, color: Color) -> Self {
        AIPlayer {
            searcher: Negaalpha::new(evaluate_fn),
            color,
        }
    }
}

impl<B: Board + Clone + std::hash::Hash + Eq> Player<B> for AIPlayer<B> {
    fn get_move(&mut self, state: &GameState<B>) -> Option<Position> {
        let search_result = self.searcher.search(state, 5, i32::MIN, i32::MAX);
        search_result.best_move.map(|mv| mv.position.unwrap())
    }
}
