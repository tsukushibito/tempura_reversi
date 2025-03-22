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

        let children = state.generate_children();
        if children.is_empty() {
            return self.evaluator.evaluate(state);
        }
        let ordered = self.order_states(&children);

        let mut best = -INF;
        let mut current_alpha = alpha;
        for child in ordered {
            let value = -self.nega_alpha_tt(&child.0, -beta, -current_alpha, depth - 1);
            best = max(best, value);
            current_alpha = max(current_alpha, value);
            if current_alpha >= beta {
                break;
            }
        }

        self.tt.store(state.clone(), depth, best, alpha, beta);
        best
    }

    fn order_states(&mut self, states: &[(S, S::Move)]) -> Vec<(S, S::Move)> {
        let mut evaluated_states: Vec<(i32, (S, S::Move))> = states
            .iter()
            .cloned()
            .map(|s| {
                let value = if let Some(v) = self.tt_snapshot.get_value(&s.0) {
                    -v + TT_BIAS
                } else {
                    -self.order_evaluator.evaluate(&s.0)
                };
                (value, s)
            })
            .collect();
        evaluated_states.sort_by(|a, b| b.0.cmp(&a.0));
        evaluated_states.into_iter().map(|(_, s)| s).collect()
    }

    fn search_best_move_at_depth(&mut self, state: &S, depth: usize) -> Option<(S::Move, i32)> {
        let children = state.generate_children();
        if children.is_empty() {
            return None;
        }
        let ordered = self.order_states(&children);

        let mut best_move_and_value = None;
        let mut best_value = -INF;
        for child in ordered {
            let value = -self.nega_alpha_tt(&child.0, -INF, INF, depth - 1);
            if value > best_value {
                best_value = value;
                best_move_and_value = Some((child.1, best_value));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
    struct DummyState {
        eval: i32,
        depth: usize,
        children: Vec<DummyState>,
    }

    impl GameState for DummyState {
        type Move = u32;
        fn generate_children(&self) -> Vec<(Self, Self::Move)> {
            self.children
                .iter()
                .enumerate()
                .map(|(i, c)| (c.clone(), i as u32))
                .collect()
        }
    }

    struct DummyEvaluator;

    impl Evaluator<DummyState> for DummyEvaluator {
        fn evaluate(&mut self, state: &DummyState) -> i32 {
            state.eval
        }
    }

    #[test]
    fn test_order_moves_with_snapshot() {
        let child1 = DummyState {
            eval: 50,
            depth: 0,
            children: vec![],
        };
        let child2 = DummyState {
            eval: 10,
            depth: 0,
            children: vec![],
        };
        let parent = DummyState {
            eval: 0,
            depth: 1,
            children: vec![child1.clone(), child2.clone()],
        };

        let mut na = NegaAlphaTT::<DummyState, DummyEvaluator, DummyEvaluator>::new(
            DummyEvaluator,
            DummyEvaluator,
        );
        na.tt_snapshot.store(child2.clone(), 0, 200, 190, 210);

        let children = parent.generate_children();
        let ordered = na.order_states(&children);
        assert_eq!(
            ordered[0].0, child2,
            "Child2 should be first due to TT bias"
        );
        assert_eq!(ordered[1].0, child1, "Child1 should be second");
    }

    #[test]
    fn test_duplicate_states_tt_hits() {
        let leaf = DummyState {
            eval: 10,
            depth: 0,
            children: vec![],
        };

        let child1 = DummyState {
            eval: 0,
            depth: 1,
            children: vec![leaf.clone()],
        };
        let child2 = DummyState {
            eval: 0,
            depth: 1,
            children: vec![leaf.clone()],
        };

        let root = DummyState {
            eval: 0,
            depth: 2,
            children: vec![child1, child2],
        };

        let mut na = NegaAlphaTT::<DummyState, DummyEvaluator, DummyEvaluator>::new(
            DummyEvaluator,
            DummyEvaluator,
        );
        let result = na.nega_alpha_tt(&root, -INF, INF, 2);
        assert_eq!(result, 10, "The final evaluation should be 10");
        assert!(na.tt.hits > 0, "TT hit count should be > 0");
    }

    #[test]
    fn test_negamax_alpha_beta() {
        // 終端状態
        let leaf1 = DummyState {
            eval: -200,
            depth: 0,
            children: vec![],
        };
        let leaf2 = DummyState {
            eval: -50,
            depth: 0,
            children: vec![],
        };
        let branch1 = DummyState {
            eval: 200,
            depth: 2,
            children: vec![leaf1, leaf2],
        };

        let leaf3 = DummyState {
            eval: 10,
            depth: 0,
            children: vec![],
        };
        let leaf4 = DummyState {
            eval: 20,
            depth: 0,
            children: vec![],
        };
        let branch2 = DummyState {
            eval: -10,
            depth: 2,
            children: vec![leaf3, leaf4],
        };

        let leaf5 = DummyState {
            eval: 0,
            depth: 0,
            children: vec![],
        };
        let leaf6 = DummyState {
            eval: 5,
            depth: 0,
            children: vec![],
        };
        let branch3 = DummyState {
            eval: 0,
            depth: 2,
            children: vec![leaf5, leaf6],
        };

        let root = DummyState {
            eval: 0,
            depth: 3,
            children: vec![branch1, branch2, branch3],
        };

        let mut na = NegaAlphaTT::<DummyState, DummyEvaluator, DummyEvaluator>::new(
            DummyEvaluator,
            DummyEvaluator,
        );
        let result = na.search_best_move(&root, 2).unwrap().1;
        assert_eq!(result, 10, "Expected root evaluation to be 10");
    }
}
