use crate::{hasher::Fnv1aHashMap, Evaluator, GameState, GameStateAndMove};
use std::cmp::max;

#[derive(Debug, Clone)]
struct TTEntry {
    depth: usize,
    value: i32,
    flag: NodeType,
}

#[derive(Debug, Clone, PartialEq)]
enum NodeType {
    Exact,
    LowerBound,
    UpperBound,
}

type TranspositionTable<S> = Fnv1aHashMap<S, TTEntry>;

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
    pub tt_hits: usize,
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
            tt_hits: 0,
            tt: Default::default(),
            tt_snapshot: Default::default(),
            evaluator,
            order_evaluator,
        }
    }

    fn nega_alpha_tt(&mut self, state: &S, mut alpha: i32, beta: i32, depth: usize) -> i32 {
        self.visited_nodes += 1;

        if depth == 0 || state.is_terminal() {
            return self.evaluator.evaluate(state);
        }

        if let Some(entry) = self.tt.get(state) {
            if entry.depth >= depth {
                self.tt_hits += 1;
                match entry.flag {
                    NodeType::Exact => return entry.value,
                    NodeType::LowerBound => alpha = max(alpha, entry.value),
                    NodeType::UpperBound => {
                        if entry.value <= alpha {
                            return entry.value;
                        }
                    }
                }
                if alpha >= beta {
                    return entry.value;
                }
            }
        }

        let children = state.generate_children();
        if children.is_empty() {
            return self.evaluator.evaluate(state);
        }
        let ordered = self.order_moves(&children);

        let mut best = -INF;
        let mut current_alpha = alpha;
        for child in ordered {
            let score = -self.nega_alpha_tt(&child.0, -beta, -current_alpha, depth - 1);
            best = max(best, score);
            current_alpha = max(current_alpha, score);
            if current_alpha >= beta {
                break;
            }
        }

        let flag = if best <= alpha {
            NodeType::UpperBound
        } else if best >= beta {
            NodeType::LowerBound
        } else {
            NodeType::Exact
        };
        self.tt.insert(
            state.clone(),
            TTEntry {
                depth,
                value: best,
                flag,
            },
        );
        best
    }

    pub fn iterative_deepening(&mut self, root: &S, max_depth: usize) -> i32 {
        let mut best_value = -INF;
        for depth in 1..=max_depth {
            self.tt.clear();
            best_value = self.nega_alpha_tt(root, -INF, INF, depth);
            self.tt_snapshot = std::mem::take(&mut self.tt);
        }
        best_value
    }

    pub fn search_best_move(&mut self, root: &S, max_depth: usize) -> S::Move {
        let mut best_move = None;
        let mut best_value = -INF;
        for depth in 1..=max_depth {
            let children = root.generate_children();
            for child in children {
                let score = -self.nega_alpha_tt(&child.0, -INF, INF, depth - 1);
                if score > best_value {
                    best_value = score;
                    best_move = Some(child.1);
                }
            }
        }
        best_move.unwrap()
    }

    fn order_moves(&mut self, states: &[GameStateAndMove<S>]) -> Vec<GameStateAndMove<S>> {
        let mut scored: Vec<(i32, GameStateAndMove<S>)> = states
            .iter()
            .cloned()
            .map(|s| {
                let score = if let Some(entry) = self.tt_snapshot.get(&s.0) {
                    -entry.value + TT_BIAS
                } else {
                    -self.order_evaluator.evaluate(&s.0)
                };
                (score, s)
            })
            .collect();
        scored.sort_by(|a, b| b.0.cmp(&a.0));
        scored.into_iter().map(|(_, s)| s).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct DummyState {
        eval: i32,
        depth: usize,
        children: Vec<DummyState>,
    }

    impl GameState for DummyState {
        type Move = u32;

        fn is_terminal(&self) -> bool {
            self.depth == 0
        }
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
        na.tt_snapshot.insert(
            child2.clone(),
            TTEntry {
                depth: 0,
                value: 200,
                flag: NodeType::Exact,
            },
        );

        let children = parent.generate_children();
        let ordered = na.order_moves(&children);
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
        assert!(na.tt_hits > 0, "TT hit count should be > 0");
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
        let result = na.iterative_deepening(&root, 2);
        assert_eq!(result, 10, "Expected root evaluation to be 10");
    }
}
