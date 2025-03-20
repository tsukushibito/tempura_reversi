use crate::{Evaluator, GameState, LookupResult, TranspositionTable};

use super::Searcher;

const INF: i32 = i32::MAX;
const TT_BIAS: i32 = 1000;

#[derive(Debug, Clone)]
pub struct NegaScoutMPC<S, E, O>
where
    S: GameState,
    E: Evaluator<S>,
    O: Evaluator<S>,
{
    pub visited_nodes: usize,
    tt: TranspositionTable<S>,
    tt_snapshot: TranspositionTable<S>,
    pub evaluator: E,
    pub order_evaluator: O,
}

impl<S, E, O> NegaScoutMPC<S, E, O>
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

    fn nega_scout(&mut self, state: &S, alpha: i32, beta: i32, depth: usize) -> i32 {
        self.visited_nodes += 1;

        if depth == 0 || state.is_terminal() {
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

        // Generate children and order them based on the order evaluator.
        let children = state.generate_children();
        if children.is_empty() {
            return self.evaluator.evaluate(state);
        }
        let ordered = self.order_states(&children);

        // Perform NegaScout search.
        let original_alpha = alpha;
        let mut best_value = -INF;
        let mut is_first_move = true;
        for child in ordered {
            let mut v;
            if is_first_move {
                v = -self.nega_scout(&child.0, -beta, -alpha, depth - 1);
            } else {
                v = -self.nega_scout(&child.0, -alpha - 1, -alpha, depth - 1);
                if alpha < v && v < beta {
                    v = -self.nega_scout(&child.0, -beta, -v, depth - 1);
                }
            }

            if v > best_value {
                best_value = v;
            }
            if best_value > alpha {
                alpha = best_value;
            }
            if alpha >= beta {
                break; // Beta cut-off.
            }

            is_first_move = false;
        }

        self.tt
            .store(state.clone(), depth, best_value, original_alpha, beta);

        best_value
    }

    fn search_best_move_at_depth(&mut self, state: &S, depth: usize) -> Option<(S::Move, i32)> {
        let children = state.generate_children();
        if children.is_empty() {
            return None;
        }
        let ordered = self.order_states(&children);

        let mut alpha = -INF;
        let beta = INF;
        let mut best_value = -INF;
        let mut best_move = ordered[0].1.clone();
        let mut is_first_move = true;
        for child in ordered {
            let mut v;
            if is_first_move {
                v = -self.nega_scout(&child.0, -beta, -alpha, depth - 1);
            } else {
                v = -self.nega_scout(&child.0, -alpha - 1, -alpha, depth - 1);
                if alpha < v && v < beta {
                    v = -self.nega_scout(&child.0, -beta, -v, depth - 1);
                }
            }

            if v > best_value {
                best_value = v;
                best_move = child.1;
            }
            if best_value > alpha {
                alpha = best_value;
            }
            if alpha >= beta {
                break; // Beta cut-off.
            }

            is_first_move = false;
        }

        Some((best_move, best_value))
    }

    fn search_best_move(&mut self, root: &S, max_depth: usize) -> Option<(S::Move, i32)> {
        self.visited_nodes = 0;
        let mut best_move_and_score = None;
        let begin_depth = if max_depth > 3 { max_depth - 3 } else { 1 };
        // let begin_depth = 1;
        for depth in begin_depth..=max_depth {
            best_move_and_score = self.search_best_move_at_depth(root, depth);
            self.tt_snapshot = std::mem::take(&mut self.tt);
        }
        best_move_and_score
    }

    fn order_states(&mut self, states: &[(S, S::Move)]) -> Vec<(S, S::Move)> {
        // Compute (score, state) tuples using TT info if available.
        let mut evaluated_states: Vec<(i32, (S, S::Move))> = states
            .iter()
            .cloned()
            .map(|s| {
                let value = if let Some(v) = self.tt_snapshot.get_value(&s.0) {
                    -v + TT_BIAS
                } else {
                    -self.order_evaluator.evaluate(&s.0)
                };
                (value, s.clone())
            })
            .collect();
        // Sort in descending order (higher score first).
        evaluated_states.sort_by(|a, b| b.0.cmp(&a.0));
        // Return only the states in sorted order.
        evaluated_states.into_iter().map(|(_, s)| s).collect()
    }
}

impl<S, E, O> Searcher<S> for NegaScoutMPC<S, E, O>
where
    S: GameState,
    E: Evaluator<S>,
    O: Evaluator<S>,
{
    fn search(&mut self, root: &S, max_depth: usize) -> Option<(S::Move, i32)> {
        self.search_best_move(root, max_depth)
    }
}
