use crate::{board::Board, game_play::GameState, Color, Position};

use super::{player::Player, search::Negaalpha};

pub struct AiPlayer<B: Board> {
    searcher: Negaalpha<B>,
    color: Color,
    // 必要に応じて他のフィールドを追加
}

impl<B: Board + Clone + std::hash::Hash + Eq> AiPlayer<B> {
    pub fn new(evaluate_fn: fn(&GameState<B>, Color) -> i32, color: Color) -> Self {
        AiPlayer {
            searcher: Negaalpha::new(evaluate_fn),
            color,
        }
    }
}

impl<B: Board + Clone + std::hash::Hash + Eq> Player<B> for AiPlayer<B> {
    fn get_move(&mut self, state: &GameState<B>) -> Option<Position> {
        let search_result = self.searcher.search(state, 5, i32::MIN + 1, i32::MAX);
        search_result.best_move.map(|mv| mv.position.unwrap())
    }
}
