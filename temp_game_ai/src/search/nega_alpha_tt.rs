use crate::hasher::Fnv1aHashMap;
use std::cmp::max;

use super::GameState;

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

pub struct NegaAlphaTT<S: GameState> {
    pub visited_nodes: usize,
    pub tt_hits: usize,
    tt: TranspositionTable<S>,
    tt_snapshot: TranspositionTable<S>,
}

impl<S: GameState> NegaAlphaTT<S> {
    pub fn new() -> Self {
        Self {
            visited_nodes: 0,
            tt_hits: 0,
            tt: Default::default(),
            tt_snapshot: Default::default(),
        }
    }

    fn nega_alpha_tt(&mut self, state: &S, mut alpha: i32, beta: i32, depth: usize) -> i32 {
        self.visited_nodes += 1;

        if depth == 0 || state.is_terminal() {
            return state.evaluate();
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
            return state.evaluate();
        }
        let ordered = self.order_moves(&children);

        let mut best = -INF;
        let mut current_alpha = alpha;
        for child in ordered {
            let score = -self.nega_alpha_tt(&child, -beta, -current_alpha, depth - 1);
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
            println!("Depth {}: best_value = {}", depth, best_value);
            self.tt_snapshot = std::mem::take(&mut self.tt);
        }
        best_value
    }

    fn order_moves(&self, states: &[S]) -> Vec<S> {
        let mut scored: Vec<(i32, S)> = states
            .iter()
            .cloned()
            .map(|s| {
                let score = if let Some(entry) = self.tt_snapshot.get(&s) {
                    -entry.value + TT_BIAS
                } else {
                    -s.order_evaluate()
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
        fn is_terminal(&self) -> bool {
            self.depth == 0
        }
        fn evaluate(&self) -> i32 {
            self.eval
        }
        fn generate_children(&self) -> Vec<Self> {
            self.children.clone()
        }
        fn order_evaluate(&self) -> i32 {
            self.evaluate()
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

        let mut na = NegaAlphaTT::<DummyState>::new();
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
        assert_eq!(ordered[0], child2, "Child2 should be first due to TT bias");
        assert_eq!(ordered[1], child1, "Child1 should be second");
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

        let mut na = NegaAlphaTT::<DummyState>::new();
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

        let mut na = NegaAlphaTT::<DummyState>::new();
        let result = na.iterative_deepening(&root, 2);
        assert_eq!(result, 10, "Expected root evaluation to be 10");
    }
}
