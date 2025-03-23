use crate::{Evaluator, GameState};

use super::Searcher;

pub struct NegaMax<S, E>
where
    S: GameState,
    E: Evaluator<S>,
{
    pub evaluator: E,
    pub visited_nodes: usize,
    phantom: std::marker::PhantomData<S>,
}

impl<S, E> NegaMax<S, E>
where
    S: GameState,
    E: Evaluator<S>,
{
    pub fn new(evaluator: E) -> Self {
        Self {
            evaluator,
            visited_nodes: 0,
            phantom: std::marker::PhantomData,
        }
    }

    fn nega_max(&mut self, state: &mut S, depth: usize) -> i32 {
        self.visited_nodes += 1;

        if depth == 0 {
            return self.evaluator.evaluate(state);
        }

        let valid_moves = state.valid_moves();
        let mut best = i32::MIN;
        for mv in valid_moves {
            state.make_move(&mv);
            let score = -self.nega_max(state, depth - 1);
            state.undo_move();
            best = best.max(score);
        }

        best
    }

    fn search_best_move(&mut self, state: &mut S, depth: usize) -> Option<(S::Move, i32)> {
        self.visited_nodes = 1;
        let mut best_move_and_score = None;
        let mut best_value = i32::MIN;
        let valid_moves = state.valid_moves();
        for mv in valid_moves {
            state.make_move(&mv);
            let score = -self.nega_max(state, depth - 1);
            state.undo_move();
            if score > best_value {
                best_value = score;
                best_move_and_score = Some((mv, best_value));
            }
        }
        best_move_and_score
    }
}

impl<S, E> Searcher<S> for NegaMax<S, E>
where
    S: GameState,
    E: Evaluator<S>,
{
    fn search(&mut self, state: &mut S, max_depth: usize) -> Option<(S::Move, i32)> {
        self.search_best_move(state, max_depth)
    }
}
