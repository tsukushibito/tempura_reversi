use std::cmp::max;

use crate::{Evaluator, GameState};

use super::Searcher;

const INF: i32 = i32::MAX;

#[derive(Debug, Clone)]
pub struct NegaAlpha<S, E>
where
    S: GameState,
    E: Evaluator<S>,
{
    pub visited_nodes: usize,
    evaluator: E,
    phantom: std::marker::PhantomData<S>,
}

impl<S, E> NegaAlpha<S, E>
where
    S: GameState,
    E: Evaluator<S>,
{
    pub fn new(evaluator: E) -> Self {
        Self {
            visited_nodes: 0,
            evaluator,
            phantom: std::marker::PhantomData,
        }
    }

    fn nega_alpha(&mut self, state: &mut S, alpha: i32, beta: i32, depth: usize) -> i32 {
        self.visited_nodes += 1;
        if depth == 0 {
            return self.evaluator.evaluate(state);
        }

        let valid_moves = state.valid_moves();
        let mut alpha = alpha;
        let mut best = -INF;
        for mv in valid_moves {
            state.make_move(&mv);
            let score = -self.nega_alpha(state, -beta, -alpha, depth - 1);
            state.undo_move();
            best = max(best, score);
            alpha = max(alpha, score);
            if alpha >= beta {
                break;
            }
        }
        best
    }

    fn search_best_move(&mut self, state: &mut S, max_depth: usize) -> Option<(S::Move, i32)> {
        let mut best_move_and_score = None;
        let mut best_value = -INF;
        for depth in 1..=max_depth {
            let valid_moves = state.valid_moves();
            for mv in valid_moves {
                state.make_move(&mv);
                let score = -self.nega_alpha(state, -INF, INF, depth - 1);
                state.undo_move();
                if score > best_value {
                    best_value = score;
                    best_move_and_score = Some((mv, best_value));
                }
            }
        }
        best_move_and_score
    }
}

impl<S, E> Searcher<S> for NegaAlpha<S, E>
where
    S: GameState,
    E: Evaluator<S>,
{
    fn search(&mut self, state: &mut S, max_depth: usize) -> Option<(S::Move, i32)> {
        self.search_best_move(state, max_depth)
    }
}
