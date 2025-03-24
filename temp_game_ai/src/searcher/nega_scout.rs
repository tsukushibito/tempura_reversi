use crate::{Evaluator, GameState, LookupResult, TranspositionTable};

use super::Searcher;

const INF: i32 = i32::MAX;
const TT_BIAS: i32 = 1000;
const TT_BIAS_DELTA: i32 = 100;

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

    fn nega_scout(&mut self, state: &mut S, alpha: i32, beta: i32, depth: usize) -> i32 {
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
        let ordered = self.order_moves(valid_moves, state, depth);

        // Perform NegaScout search.
        let original_alpha = alpha;
        let mut best_value = -INF;
        let mut is_first_move = true;
        for mv in ordered {
            state.make_move(&mv);
            let mut v;
            if is_first_move {
                v = -self.nega_scout(state, -beta, -alpha, depth - 1);
            } else {
                v = -self.nega_scout(state, -alpha - 1, -alpha, depth - 1);
                if alpha < v && v < beta {
                    v = -self.nega_scout(state, -beta, -v, depth - 1);
                }
            }
            state.undo_move();

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

    fn search_best_move_at_depth(&mut self, state: &mut S, depth: usize) -> Option<(S::Move, i32)> {
        let valid_moves = state.valid_moves();
        let ordered = self.order_moves(valid_moves, state, depth);

        let mut alpha = -INF;
        let beta = INF;
        let mut best_value = -INF;
        let mut best_move = None;
        let mut is_first_move = true;
        for mv in ordered {
            state.make_move(&mv);
            let mut v;
            if is_first_move {
                v = -self.nega_scout(state, -beta, -alpha, depth - 1);
            } else {
                v = -self.nega_scout(state, -alpha - 1, -alpha, depth - 1);
                if alpha < v && v < beta {
                    v = -self.nega_scout(state, -beta, -v, depth - 1);
                }
            }
            state.undo_move();

            if v > best_value {
                best_value = v;
                best_move = Some(mv);
            }
            if best_value > alpha {
                alpha = best_value;
            }
            if alpha >= beta {
                break; // Beta cut-off.
            }

            is_first_move = false;
        }

        if let Some(mv) = best_move {
            Some((mv, best_value))
        } else {
            None
        }
    }

    fn search_best_move(&mut self, state: &mut S, max_depth: usize) -> Option<(S::Move, i32)> {
        self.visited_nodes = 0;
        let mut best_move_and_score = None;
        let begin_depth = if max_depth > 3 { max_depth - 3 } else { 1 };
        // let begin_depth = 1;
        for depth in begin_depth..=max_depth {
            best_move_and_score = self.search_best_move_at_depth(state, depth);
            self.tt_snapshot = std::mem::take(&mut self.tt);
        }
        best_move_and_score
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
    fn search(&mut self, state: &mut S, max_depth: usize) -> Option<(S::Move, i32)> {
        self.search_best_move(state, max_depth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::{DummyEvaluator, DummyGame, DummyMove, OptimalOrderingEvaluator};

    #[test]
    fn test_negascout_with_dummy_game() {
        // Use DummyEvaluator for evaluation, and OptimalOrderingEvaluator for move
        // ordering.
        let evaluator = DummyEvaluator;
        let order_evaluator = OptimalOrderingEvaluator;
        let mut searcher = NegaScout::new(evaluator, order_evaluator);
        let mut game = DummyGame::new();

        // Use search_best_move_at_depth to search at depth 3.
        let result = searcher.search_best_move_at_depth(&mut game, 3);
        // Evaluations for each move based on pre-analysis:
        //   - DummyMove::A -> -7
        //   - DummyMove::B -> -16
        //   - DummyMove::C -> -25
        // Thus, the best move should be DummyMove::A with a score of -7.
        assert_eq!(result, Some((DummyMove::A, -7)));

        // Verify that visited_nodes equals 19 at this point.
        assert_eq!(searcher.visited_nodes, 19);

        // Furthermore, calling search() with iterative deepening should yield the same
        // result.
        let result_iter = searcher.search(&mut game, 3);
        assert_eq!(result_iter, Some((DummyMove::A, -7)));

        // In iterative deepening, results accumulate from each depth.
        // Verify that the total visited nodes count is at most 54 (for example, the sum
        // for depths 1, 2, and 3).
        println!("Visited nodes: {}", searcher.visited_nodes);
        assert!(
            searcher.visited_nodes <= 54,
            "Visited nodes: {}",
            searcher.visited_nodes
        );
    }
}
