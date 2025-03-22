use crate::{Evaluator, GameState, LookupResult, TranspositionTable};

use super::Searcher;

const INF: i32 = i32::MAX;
const TT_BIAS: i32 = 1000;

#[derive(Debug, Clone)]
pub struct NegaScout<S, E, O>
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

impl<S, E, O> NegaScout<S, E, O>
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

impl<S, E, O> Searcher<S> for NegaScout<S, E, O>
where
    S: GameState,
    E: Evaluator<S>,
    O: Evaluator<S>,
{
    fn search(&mut self, root: &S, max_depth: usize) -> Option<(S::Move, i32)> {
        self.search_best_move(root, max_depth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // DummyState implements the GameState trait.
    #[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
    struct DummyState {
        eval: i32,    // Terminal state evaluation value.
        depth: usize, // If depth == 0 then terminal.
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

        let mut ns = NegaScout::<DummyState, DummyEvaluator, DummyEvaluator>::new(
            DummyEvaluator,
            DummyEvaluator,
        );

        // Set TT snapshot so that child2 is favored (its TT value is 200).
        ns.tt_snapshot.store(child2.clone(), 0, 200, 190, 210);

        // order_moves() should return children sorted in descending order.
        let children = root.generate_children();
        let ordered = ns.order_states(&children);
        // child2's ordering score = 200 + TT_BIAS, and child1's score = child1.order_evaluate() (80).
        assert_eq!(
            ordered[0].0, child2,
            "Child2 should be first due to TT bias"
        );
        assert_eq!(ordered[1].0, child1, "Child1 should be second");
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

        let mut ns = NegaScout::<DummyState, DummyEvaluator, DummyEvaluator>::new(
            DummyEvaluator,
            DummyEvaluator,
        );
        // Simulate TT snapshot: For child2, TT entry with high value (e.g., 200).
        ns.tt_snapshot.store(child2.clone(), 0, 200, 190, 210);
        // order_moves takes a slice of states and returns sorted Vec.
        let children = parent.generate_children();
        let ordered = ns.order_states(&children);
        // child2's ordering score = 200 + TT_BIAS, child1's ordering score = child1.order_evaluate() (50).
        // Therefore, child2 should be first.
        assert_eq!(
            ordered[0].0, child2,
            "Child2 should be ordered first due to TT snapshot bias"
        );
        assert_eq!(ordered[1].0, child1, "Child1 should come second");
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

        let mut ns = NegaScout::<DummyState, DummyEvaluator, DummyEvaluator>::new(
            DummyEvaluator,
            DummyEvaluator,
        );

        // Use standard search window (here max_depth = 2)
        let result = ns.nega_scout(&root, -INF, INF, 2);
        // Terminal state's evaluation is 10, so final evaluation should be 10.
        assert_eq!(result, 10, "The final evaluation should be 10");

        // Since there are duplicate states, TT hit should occur.
        // Expected TT entry count is about 3 (leaf, child1/child2, root).
        assert!(ns.tt.hits > 0, "TT hit count should be > 0");
        println!("Visited nodes: {}", ns.visited_nodes);
        println!("TT hits: {}", ns.tt.hits);
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

        let mut ns = NegaScout::<DummyState, DummyEvaluator, DummyEvaluator>::new(
            DummyEvaluator,
            DummyEvaluator,
        );

        let result = ns.search_best_move(&root, 2);
        assert!(result.is_some(), "Best move should be found");

        assert_eq!(result.unwrap().0, 1, "Best move should be 1 (branch2)");

        assert!(
            ns.visited_nodes == 10,
            "Visited nodes should be relatively low due to pruning"
        );
    }
}
