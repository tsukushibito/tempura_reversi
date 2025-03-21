use crate::{Evaluator, GameState};

use super::Searcher;

pub struct NegaMax<S, E>
where
    S: GameState,
    E: Evaluator<S>,
{
    pub evaluator: E,
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
            phantom: std::marker::PhantomData,
        }
    }

    fn nega_max(&mut self, state: &S, depth: usize) -> i32 {
        if depth == 0 || state.is_terminal() {
            return self.evaluator.evaluate(state);
        }

        let children = state.generate_children();
        if children.is_empty() {
            return self.evaluator.evaluate(state);
        }

        let mut best = i32::MIN;

        for child in children {
            let score = -self.nega_max(&child.0, depth - 1);
            best = best.max(score);
        }

        best
    }

    fn search_best_move(&mut self, state: &S, depth: usize) -> Option<(S::Move, i32)> {
        let mut best_move_and_score = None;
        let mut best_value = i32::MIN;
        let children = state.generate_children();
        for child in children {
            let score = -self.nega_max(&child.0, depth - 1);
            if score > best_value {
                best_value = score;
                best_move_and_score = Some((child.1, best_value));
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
    fn search(&mut self, state: &S, max_depth: usize) -> Option<(S::Move, i32)> {
        self.search_best_move(state, max_depth)
    }
}
