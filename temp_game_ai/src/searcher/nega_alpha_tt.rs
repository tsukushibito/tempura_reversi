use crate::{Evaluator, GameState, LookupResult, TranspositionTable};
use std::cmp::max;

use super::Searcher;

const INF: i32 = i32::MAX;
const TT_BIAS: i32 = 1000;
const TT_BIAS_DELTA: i32 = 100;

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

    fn nega_alpha_tt(&mut self, state: &mut S, alpha: i32, beta: i32, depth: usize) -> i32 {
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
        let ordered = self.order_moves(valid_moves, state, depth);

        let mut best = -INF;
        let mut current_alpha = alpha;
        for mv in ordered {
            state.make_move(&mv);
            let value = -self.nega_alpha_tt(state, -beta, -current_alpha, depth - 1);
            state.undo_move();
            best = max(best, value);
            current_alpha = max(current_alpha, value);
            if current_alpha >= beta {
                break;
            }
        }

        self.tt.store(state.clone(), depth, best, alpha, beta);
        best
    }

    fn order_moves(&mut self, moves: Vec<S::Move>, state: &mut S, depth: usize) -> Vec<S::Move> {
        let mut evaluated_states: Vec<(i32, S::Move)> = moves
            .into_iter()
            .map(|mv| {
                state.make_move(&mv);
                let entry = self.tt_snapshot.get_entry(state);
                let value = match entry {
                    Some(e) if e.depth >= depth => match e.node_type {
                        crate::NodeType::Exact => e.value + TT_BIAS,
                        crate::NodeType::LowerBound => e.value + TT_BIAS - TT_BIAS_DELTA,
                        crate::NodeType::UpperBound => e.value + TT_BIAS - 2 * TT_BIAS_DELTA,
                    },
                    _ => -self.order_evaluator.evaluate(&state),
                };
                state.undo_move();
                (value, mv)
            })
            .collect();
        evaluated_states.sort_by(|a, b| b.0.cmp(&a.0));
        evaluated_states.into_iter().map(|(_, m)| m).collect()
    }

    fn search_best_move_at_depth(&mut self, state: &mut S, depth: usize) -> Option<(S::Move, i32)> {
        let valid_moves = state.valid_moves();
        let ordered = self.order_moves(valid_moves, state, depth);

        let mut best_move_and_value = None;
        let mut best_value = -INF;
        for mv in ordered {
            state.make_move(&mv);
            let value = -self.nega_alpha_tt(state, -INF, INF, depth - 1);
            state.undo_move();
            if value > best_value {
                best_value = value;
                best_move_and_value = Some((mv, best_value));
            }
        }

        best_move_and_value
    }

    fn search_best_move(&mut self, root: &mut S, max_depth: usize) -> Option<(S::Move, i32)> {
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
    fn search(&mut self, state: &mut S, max_depth: usize) -> Option<(S::Move, i32)> {
        self.search_best_move(state, max_depth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::{DummyEvaluator, DummyGame, DummyMove, OptimalOrderingEvaluator};

    #[test]
    fn test_negaalpha_tt_with_dummy_game() {
        // Use the same DummyEvaluator for both evaluation and ordering
        // evaluation functions
        let evaluator = DummyEvaluator;
        let order_evaluator = OptimalOrderingEvaluator;
        let mut searcher = NegaAlphaTT::new(evaluator, order_evaluator);
        let mut game = DummyGame::new();

        // Execute search with depth 3 using the
        // search_best_move_at_depth method
        let result = searcher.search_best_move_at_depth(&mut game, 3);
        // According to prior analysis, the evaluations for each initial move
        // should be as follows:
        //   - DummyMove::A -> -7
        //   - DummyMove::B -> -16
        //   - DummyMove::C -> -25
        // Therefore, the best move should be DummyMove::A with score -7
        assert_eq!(result, Some((DummyMove::A, -7)));

        // Visited nodes should be 27
        assert_eq!(searcher.visited_nodes, 27);

        // Execute search with depth 3 using the search method
        // (iterative deepening)
        let result = searcher.search(&mut game, 3);
        assert_eq!(result, Some((DummyMove::A, -7)));

        // Visited nodes should be less than or equal to 54
        // 54 is the maximum number of nodes visited when the
        // search_best_move_at_depth method is called for each depth from 1 to 3
        println!("Visited nodes: {}", searcher.visited_nodes);
        assert!(
            searcher.visited_nodes <= 54,
            "Visited nodes: {}",
            searcher.visited_nodes
        );
    }
}
