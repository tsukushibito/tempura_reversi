use crate::{Evaluator, GameState, LookupResult, TranspositionTable};
use std::cmp::max;

use super::Searcher;

const INF: i32 = i32::MAX;
const TT_BIAS: i32 = 1000;

#[derive(Debug, Clone)]
pub struct NegaAlphaTT<S, E, O>
where
    S: GameState,
    E: Evaluator<S>,
    O: Evaluator<S>,
{
    pub visited_nodes: usize,
    tt: TranspositionTable<S>,
    tt_snapshot: TranspositionTable<S>,
    evaluator: E,
    order_evaluator: O,
}

impl<S, E, O> NegaAlphaTT<S, E, O>
where
    S: GameState,
    E: Evaluator<S>,
    O: Evaluator<S>,
{
    pub fn new(evaluator: E, order_evaluator: O) -> Self {
        Self {
            visited_nodes: 0,
            tt: Default::default(),
            tt_snapshot: Default::default(),
            evaluator,
            order_evaluator,
        }
    }

    fn nega_alpha_tt(&mut self, state: &S, alpha: i32, beta: i32, depth: usize) -> i32 {
        self.visited_nodes += 1;

        if depth == 0 {
            return self.evaluator.evaluate(state);
        }

        let mut alpha = alpha;
        let mut beta = beta;
        let r = self.tt.lookup(state, alpha, beta, depth);
        match r {
            LookupResult::Value(v) => return v,
            LookupResult::AlphaBeta(a, b) => {
                alpha = a;
                beta = b;
            }
        }

        let valid_moves = state.valid_moves();
        if valid_moves.is_empty() {
            return self.evaluator.evaluate(state);
        }
        let children: Vec<S> = valid_moves
            .into_iter()
            .map(|m| {
                let mut s = state.clone();
                s.make_move(&m);
                s
            })
            .collect();
        let ordered = self.order_states(children);

        let mut best = -INF;
        let mut current_alpha = alpha;
        for child in ordered {
            let value = -self.nega_alpha_tt(&child.1, -beta, -current_alpha, depth - 1);
            best = max(best, value);
            current_alpha = max(current_alpha, value);
            if current_alpha >= beta {
                break;
            }
        }

        self.tt.store(state.clone(), depth, best, alpha, beta);
        best
    }

    fn order_states(&mut self, states: Vec<S>) -> Vec<(usize, S)> {
        let mut evaluated_states: Vec<(i32, (usize, S))> = states
            .into_iter()
            .enumerate()
            .map(|(index, state)| {
                let value = if let Some(v) = self.tt_snapshot.get_value(&state) {
                    -v + TT_BIAS
                } else {
                    -self.order_evaluator.evaluate(&state)
                };
                (value, (index, state))
            })
            .collect();
        evaluated_states.sort_by(|a, b| b.0.cmp(&a.0));
        evaluated_states.into_iter().map(|(_, s)| s).collect()
    }

    fn search_best_move_at_depth(&mut self, state: &S, depth: usize) -> Option<(S::Move, i32)> {
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
            return None;
        }
        let ordered = self.order_states(children);

        let mut best_move_and_value = None;
        let mut best_value = -INF;
        for child in ordered {
            let value = -self.nega_alpha_tt(&child.1, -INF, INF, depth - 1);
            if value > best_value {
                best_value = value;
                let mv = valid_moves[child.0].clone();
                best_move_and_value = Some((mv, best_value));
            }
        }

        best_move_and_value
    }

    fn search_best_move(&mut self, root: &S, max_depth: usize) -> Option<(S::Move, i32)> {
        self.visited_nodes = 0;
        let mut best_move_and_value = None;
        let begin_depth = if max_depth > 3 { max_depth - 3 } else { 1 };
        // let begin_depth = 1;
        for depth in begin_depth..=max_depth {
            best_move_and_value = self.search_best_move_at_depth(root, depth);
            self.tt_snapshot = std::mem::take(&mut self.tt);
        }
        best_move_and_value
    }
}

impl<S, E, O> Searcher<S> for NegaAlphaTT<S, E, O>
where
    S: GameState,
    E: Evaluator<S>,
    O: Evaluator<S>,
{
    fn search(&mut self, state: &S, max_depth: usize) -> Option<(S::Move, i32)> {
        self.search_best_move(state, max_depth)
    }
}
