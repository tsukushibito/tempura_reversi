use std::cmp::max;

use super::{Evaluator, GameState};

const INF: i32 = i32::MAX;

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

    fn nega_alpha(&mut self, state: &S, mut alpha: i32, beta: i32, depth: usize) -> i32 {
        self.visited_nodes += 1;
        if depth == 0 || state.is_terminal() {
            return self.evaluator.evaluate(state);
        }

        let children = state.generate_children();
        if children.is_empty() {
            return self.evaluator.evaluate(state);
        }

        let mut best = -INF;
        for child in children {
            let score = -self.nega_alpha(&child.0, -beta, -alpha, depth - 1);
            best = max(best, score);
            alpha = max(alpha, score);
            if alpha >= beta {
                break; // βカット
            }
        }
        best
    }

    pub fn iterative_deepening(&mut self, root: &S, max_depth: usize) -> i32 {
        let mut best_value = -INF;
        for depth in 1..=max_depth {
            best_value = self.nega_alpha(root, -INF, INF, depth);
            println!("Depth {}: best_value = {}", depth, best_value);
        }
        best_value
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
        fn evaluate(&self, state: &DummyState) -> i32 {
            state.eval
        }

        fn order_evaluate(&self, state: &DummyState) -> i32 {
            state.eval
        }
    }

    #[test]
    fn test_simple_negalpha() {
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
            children: vec![child1, child2],
        };

        let mut ns = NegaAlpha::<DummyState, DummyEvaluator>::new(DummyEvaluator);
        let result = ns.iterative_deepening(&root, 1);
        assert_eq!(result, -10, "The evaluation should be -10");
    }

    #[test]
    fn test_complex_tree() {
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

        let mut ns = NegaAlpha::<DummyState, DummyEvaluator>::new(DummyEvaluator);
        let result = ns.iterative_deepening(&root, 2);
        assert_eq!(result, 10, "Expected root evaluation to be 10");
    }
}
