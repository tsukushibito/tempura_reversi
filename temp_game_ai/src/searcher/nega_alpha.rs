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

    fn nega_alpha(&mut self, state: &S, alpha: i32, beta: i32, depth: usize) -> i32 {
        self.visited_nodes += 1;
        if depth == 0 {
            return self.evaluator.evaluate(state);
        }

        let valid_moves = state.valid_moves();
        let children: Vec<S> = valid_moves
            .iter()
            .map(|m| {
                let mut s = state.clone();
                s.make_move(m);
                s
            })
            .collect();
        if children.is_empty() {
            return self.evaluator.evaluate(state);
        }

        let mut alpha = alpha;
        let mut best = -INF;
        for child in children {
            let score = -self.nega_alpha(&child, -beta, -alpha, depth - 1);
            best = max(best, score);
            alpha = max(alpha, score);
            if alpha >= beta {
                break;
            }
        }
        best
    }

    fn search_best_move(&mut self, state: &S, max_depth: usize) -> Option<(S::Move, i32)> {
        let mut best_move_and_score = None;
        let mut best_value = -INF;
        for depth in 1..=max_depth {
            let valid_moves = state.valid_moves();
            let children: Vec<S> = valid_moves
                .iter()
                .map(|m| {
                    let mut s = state.clone();
                    s.make_move(m);
                    s
                })
                .collect();
            for i in 0..children.len() {
                let child = &children[i];
                let score = -self.nega_alpha(child, -INF, INF, depth - 1);
                if score > best_value {
                    best_value = score;
                    let mv = valid_moves[i].clone();
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
    fn search(&mut self, state: &S, max_depth: usize) -> Option<(S::Move, i32)> {
        self.search_best_move(state, max_depth)
    }
}
