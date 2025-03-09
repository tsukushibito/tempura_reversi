use std::cmp::max;
use std::collections::HashMap;

pub trait GameState: Clone + Eq + std::hash::Hash {
    fn is_terminal(&self) -> bool;
    fn evaluate(&self) -> i32;
    fn generate_children(&self) -> Vec<Self>;
    fn order_evaluate(&self) -> i32;
}

/// TTEntry stores the search depth, evaluation value, and node type.
#[derive(Debug, Clone)]
struct TTEntry {
    depth: usize,
    value: i32,
    flag: NodeType,
}

#[derive(Debug, Clone, PartialEq)]
enum NodeType {
    Exact,
    LowerBound, // Fail-high
    UpperBound, // Fail-low
}

type TranspositionTable<S> = HashMap<S, TTEntry>;

const INF: i32 = i32::MAX;
const TT_BIAS: i32 = 1000;

pub struct NegaScout<S: GameState> {
    pub visited_nodes: usize,
    pub tt_hits: usize,
    tt: TranspositionTable<S>,
    tt_snapshot: TranspositionTable<S>,
}

impl<S: GameState> NegaScout<S> {
    pub fn new() -> Self {
        Self {
            visited_nodes: 0,
            tt_hits: 0,
            tt: HashMap::new(),
            tt_snapshot: HashMap::new(),
        }
    }

    fn negascout(&mut self, state: &S, mut alpha: i32, beta: i32, depth: usize) -> i32 {
        self.visited_nodes += 1;

        if depth == 0 || state.is_terminal() {
            return state.evaluate();
        }

        // Transposition table lookup.
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
        let mut ordered = self.order_moves(&children);

        // Process the first child.
        let first = ordered.remove(0);
        let mut v = -self.negascout(&first, -beta, -alpha, depth - 1);
        let mut max_value = v;
        if beta <= v {
            self.tt.insert(
                state.clone(),
                TTEntry {
                    depth,
                    value: v,
                    flag: NodeType::LowerBound,
                },
            );
            return v;
        }
        if alpha < v {
            alpha = v;
        }

        // Process remaining children.
        for child in ordered {
            v = -self.negascout(&child, -alpha - 1, -alpha, depth - 1);
            if beta <= v {
                self.tt.insert(
                    state.clone(),
                    TTEntry {
                        depth,
                        value: v,
                        flag: NodeType::LowerBound,
                    },
                );
                return v;
            }
            if alpha < v {
                alpha = v;
                v = -self.negascout(&child, -beta, -alpha, depth - 1);
                if beta <= v {
                    self.tt.insert(
                        state.clone(),
                        TTEntry {
                            depth,
                            value: v,
                            flag: NodeType::LowerBound,
                        },
                    );
                    return v;
                }
                if alpha < v {
                    alpha = v;
                }
            }
            if max_value < v {
                max_value = v;
            }
        }

        // Determine the flag for TT entry based on the result.
        let flag = if max_value <= alpha {
            NodeType::UpperBound
        } else if max_value >= beta {
            NodeType::LowerBound
        } else {
            NodeType::Exact
        };
        self.tt.insert(
            state.clone(),
            TTEntry {
                depth,
                value: max_value,
                flag,
            },
        );
        max_value
    }

    /// Iterative deepening search from depth = 1 to max_depth.
    pub fn iterative_deepening(&mut self, root: &S, max_depth: usize) -> i32 {
        let mut best_value = -INF;
        for depth in 1..=max_depth {
            self.tt.clear();
            best_value = self.negascout(root, -INF, INF, depth);
            println!("Depth {}: best_value = {}", depth, best_value);
            self.tt_snapshot = std::mem::take(&mut self.tt);
        }
        best_value
    }

    pub fn order_moves(&self, states: &[S]) -> Vec<S> {
        // Compute (score, state) tuples using TT info if available.
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
        // Sort in descending order (higher score first).
        scored.sort_by(|a, b| b.0.cmp(&a.0));
        // Return only the states in sorted order.
        scored.into_iter().map(|(_, s)| s).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // DummyState implements the GameState trait.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct DummyState {
        eval: i32,    // Terminal state evaluation value.
        depth: usize, // If depth == 0 then terminal.
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
            // For move ordering, simply use evaluate() here.
            self.evaluate()
        }
    }

    /// Test that move ordering uses the TT snapshot and that pruning occurs.
    #[test]
    fn test_move_ordering_and_pruning() {
        // Construct a tree:
        //          root (depth = 1)
        //          /        \
        //     child1       child2
        // child1: terminal with eval = 80.
        // child2: terminal with eval = 10.
        // Normally, order_evaluate() would yield 80 for child1 and 10 for child2.
        // However, we simulate a TT snapshot entry for child2 with a high value (e.g., 200),
        // so that move ordering should favor child2.
        let child1 = DummyState {
            eval: 80,
            depth: 0,
            children: vec![],
        };
        let child2 = DummyState {
            eval: 10,
            depth: 0,
            children: vec![],
        };
        let root = DummyState {
            eval: 0,
            depth: 1,
            children: vec![child1.clone(), child2.clone()],
        };

        let mut ns = NegaScout::<DummyState>::new();

        // Set TT snapshot so that child2 is favored (its TT value is 200).
        ns.tt_snapshot.insert(
            child2.clone(),
            TTEntry {
                depth: 0,
                value: 200,
                flag: NodeType::Exact,
            },
        );

        // order_moves() should return children sorted in descending order.
        let children = root.generate_children();
        let ordered = ns.order_moves(&children);
        // child2's ordering score = 200 + TT_BIAS, and child1's score = child1.order_evaluate() (80).
        assert_eq!(ordered[0], child2, "Child2 should be first due to TT bias");
        assert_eq!(ordered[1], child1, "Child1 should be second");
    }

    /// Test that order_moves uses the TT snapshot for move ordering.
    #[test]
    fn test_order_moves_with_snapshot() {
        // Create a parent state with two terminal children.
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

        let mut ns = NegaScout::<DummyState>::new();
        // Simulate TT snapshot: For child2, TT entry with high value (e.g., 200).
        ns.tt_snapshot.insert(
            child2.clone(),
            TTEntry {
                depth: 0,
                value: 200,
                flag: NodeType::Exact,
            },
        );
        // order_moves takes a slice of states and returns sorted Vec.
        let children = parent.generate_children();
        let ordered = ns.order_moves(&children);
        // child2's ordering score = 200 + TT_BIAS, child1's ordering score = child1.order_evaluate() (50).
        // Therefore, child2 should be first.
        assert_eq!(
            ordered[0], child2,
            "Child2 should be ordered first due to TT snapshot bias"
        );
        assert_eq!(ordered[1], child1, "Child1 should come second");
    }

    /// Test duplicate states in TT and TT hit count.
    #[test]
    fn test_duplicate_states_tt_hits() {
        // Create a terminal state "leaf" (depth = 0, eval = 10).
        let leaf = DummyState {
            eval: 10,
            depth: 0,
            children: vec![],
        };

        // Create two branches (child1 and child2) at depth = 1, both having the same leaf.
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

        // Root state at depth = 2 with two children.
        let root = DummyState {
            eval: 0,
            depth: 2,
            children: vec![child1, child2],
        };

        let mut ns = NegaScout::<DummyState>::new();

        // Use standard search window (here max_depth = 2)
        let result = ns.negascout(&root, -INF, INF, 2);
        // Terminal state's evaluation is 10, so final evaluation should be 10.
        assert_eq!(result, 10, "The final evaluation should be 10");

        // Since there are duplicate states, TT hit should occur.
        // Expected TT entry count is about 3 (leaf, child1/child2, root).
        assert!(ns.tt_hits > 0, "TT hit count should be > 0");
        println!("Visited nodes: {}", ns.visited_nodes);
        println!("TT hits: {}", ns.tt_hits);
    }

    /// Test tree structure:
    ///
    ///               Root (depth=3)
    ///              /      |      \
    ///  Branch1(depth=2)  Branch2(depth=2)  Branch3(depth=2)
    ///      /      \          /       \          /       \
    /// Leaf1     Leaf2   Leaf3     Leaf4    Leaf5     Leaf6
    /// (eval=-200) (eval=-50) (eval=10) (eval=20) (eval=0) (eval=5)
    #[test]
    fn test_pruning_and_move_ordering() {
        // Leaf states (terminal)
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

        let mut ns = NegaScout::<DummyState>::new();

        let result = ns.iterative_deepening(&root, 2);
        assert_eq!(
            result, 10,
            "Expected root evaluation to be 10 due to pruning"
        );

        assert!(
            ns.visited_nodes == 12,
            "Visited nodes should be relatively low due to pruning"
        );
    }
}
