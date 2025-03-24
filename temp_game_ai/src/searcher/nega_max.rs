use crate::{Evaluator, GameState};

use super::Searcher;

pub struct NegaMax<S, E>
where
    S: GameState,
    E: Evaluator<S>,
{
    pub evaluator: E,
    pub visited_nodes: usize,
    phantom: std::marker::PhantomData<S>,
}

impl<S, E> NegaMax<S, E>
where
    S: GameState,
    E: Evaluator<S>,
{
    pub fn new(evaluator: E) -> Self {
        Self {
            evaluator,
            visited_nodes: 0,
            phantom: std::marker::PhantomData,
        }
    }

    fn nega_max(&mut self, state: &mut S, depth: usize) -> i32 {
        self.visited_nodes += 1;

        if depth == 0 {
            return self.evaluator.evaluate(state);
        }

        let valid_moves = state.valid_moves();
        let mut best = i32::MIN;
        for mv in valid_moves {
            state.make_move(&mv);
            let score = -self.nega_max(state, depth - 1);
            state.undo_move();
            best = best.max(score);
        }

        best
    }

    fn search_best_move(&mut self, state: &mut S, depth: usize) -> Option<(S::Move, i32)> {
        self.visited_nodes = 1;
        let mut best_move_and_score = None;
        let mut best_value = i32::MIN;
        let valid_moves = state.valid_moves();
        for mv in valid_moves {
            state.make_move(&mv);
            let score = -self.nega_max(state, depth - 1);
            state.undo_move();
            if score > best_value {
                best_value = score;
                best_move_and_score = Some((mv, best_value));
            }
        }
        best_move_and_score
    }
}

impl<S, E> Searcher<S> for NegaMax<S, E>
where
    S: GameState,
    E: Evaluator<S>,
{
    fn search(&mut self, state: &mut S, max_depth: usize) -> Option<(S::Move, i32)> {
        self.search_best_move(state, max_depth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{DummyEvaluator, DummyGame, DummyMove};

    #[test]
    fn test_negamax_with_dummy_tree() {
        // DummyEvaluator returns unique values between 1 and 27 as computed by DummyGame's compute_score
        let evaluator = DummyEvaluator;
        let mut searcher = NegaMax::new(evaluator);
        let mut game = DummyGame::new();

        // Perform a search with depth 3
        let result = searcher.search(&mut game, 3);
        // According to the analysis, the results for each move are:
        //  - DummyMove::A -> -7
        //  - DummyMove::B -> -16
        //  - DummyMove::C -> -25
        // Therefore, the best move should be DummyMove::A with an evaluation score of -7
        assert_eq!(result, Some((DummyMove::A, -7)));

        // let nodes = 1 + perft(&mut game, 1) + perft(&mut game, 2) + perft(&mut game, 3);
        // nodes = 1 + 3 + 9 + 27 = 40
        // It also verifies that the number of visited nodes is 40
        assert_eq!(
            searcher.visited_nodes, 40,
            "Visited nodes: {}",
            searcher.visited_nodes
        );
    }
}
