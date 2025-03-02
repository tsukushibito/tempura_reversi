use std::collections::HashMap;
use std::hash::Hasher;
use std::i32;
use std::{cmp, hash::Hash};

use super::Strategy;
use crate::evaluator::{EvaluationFunction, MobilityEvaluator};
use rand::rng;
use rand_distr::{Distribution, Normal};
use temp_reversi_core::{Bitboard, Game, Player, Position};

const CACHE_HIT_BONUS: i32 = 1000;

#[derive(Clone, Copy, PartialEq, Eq)]
struct SearchState {
    board: Bitboard,
    current_player: Player,
}

const FNV_OFFSET: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

/// Hashes a Player to a u64 using FNV hash parameters.
fn hash_player(player: Player) -> u64 {
    let mut hash = FNV_OFFSET;
    let player_byte: u8 = match player {
        Player::Black => 0,
        Player::White => 1,
    };
    hash ^= player_byte as u64;
    hash = hash.wrapping_mul(FNV_PRIME);
    hash
}

impl SearchState {
    fn new(board: Bitboard, current_player: Player) -> Self {
        Self {
            board,
            current_player,
        }
    }
}

impl Hash for SearchState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.board.get_hash().hash(state);
        let player_hash = hash_player(self.current_player);
        player_hash.hash(state);
    }
}

/// Helper function: applies the specified move and returns a new search state.
fn apply_move_state(state: &SearchState, pos: Position) -> Option<SearchState> {
    let mut new_board = state.board.clone();
    if new_board.apply_move(pos, state.current_player).is_ok() {
        Some(SearchState {
            board: new_board,
            current_player: state.current_player.opponent(),
        })
    } else {
        None
    }
}

/// Strategy implementing Negalpha search with a transposition table and move ordering.
#[derive(Clone)]
pub struct NegaAlphaTTStrategy<E: EvaluationFunction + Send + Sync> {
    evaluator: E,
    transposition_table: HashMap<SearchState, i32>,
    former_transposition_table: HashMap<SearchState, i32>,
    visited_nodes: u64,
    max_depth: i32,

    normal: Normal<f64>,
}

impl<E: EvaluationFunction + Send + Sync> NegaAlphaTTStrategy<E> {
    /// Constructor.
    pub fn new(evaluator: E, max_depth: i32, sigma: f64) -> Self {
        Self {
            evaluator,
            transposition_table: HashMap::new(),
            former_transposition_table: HashMap::new(),
            visited_nodes: 0,
            max_depth,
            normal: Normal::new(0.0, sigma).unwrap(),
        }
    }

    /// Calculates move ordering value using MobilityEvaluator.
    fn calc_move_ordering_value(&self, state: &SearchState) -> i32 {
        if let Some(&score) = self.former_transposition_table.get(state) {
            CACHE_HIT_BONUS - score
        } else {
            // Use MobilityEvaluator for evaluation.
            let evaluator = MobilityEvaluator;
            -evaluator.evaluate(&state.board, state.current_player)
        }
    }

    /// Negalpha search using transposition table and move ordering (recursive function).
    fn nega_alpha_transpose(
        &mut self,
        state: SearchState,
        depth: i32,
        passed: bool,
        alpha: i32,
        beta: i32,
    ) -> i32 {
        self.visited_nodes += 1;

        if depth == 0 {
            let mut rng = rng();
            let fluctuation = self.normal.sample(&mut rng) as i32;
            return self.evaluator.evaluate(&state.board, state.current_player) + fluctuation;
        }

        if let Some(&score) = self.transposition_table.get(&state) {
            return score;
        }

        let valid_moves = state.board.valid_moves(state.current_player);
        if valid_moves.is_empty() {
            if passed {
                // Game over
                return self.evaluator.evaluate(&state.board, state.current_player);
            }

            // Pass
            let next_state = SearchState::new(state.board.clone(), state.current_player.opponent());
            return -self.nega_alpha_transpose(next_state, depth, true, -beta, -alpha);
        }

        // Generate children and sort them by move ordering value
        let mut children: Vec<(Position, SearchState, i32)> = Vec::new();
        for pos in valid_moves {
            if let Some(child_state) = apply_move_state(&state, pos) {
                let ordering = self.calc_move_ordering_value(&child_state);
                children.push((pos, child_state, ordering));
            }
        }
        if children.len() >= 2 {
            children.sort_by(|a, b| b.2.cmp(&a.2));
        }

        let mut value = i32::MIN + 1;
        let mut alpha_local = alpha;
        for (_, child_state, _) in children.iter() {
            let score =
                -self.nega_alpha_transpose(*child_state, depth - 1, false, -beta, -alpha_local);
            if score >= beta {
                self.transposition_table.insert(state, score);
                return score;
            }
            if score > value {
                value = score;
            }
            if score > alpha_local {
                alpha_local = score;
            }
        }
        self.transposition_table.insert(state, value);
        value
    }

    /// Finds the best move using iterative deepening search.
    fn search(&mut self, state: SearchState) -> Option<Position> {
        self.visited_nodes = 0;
        self.transposition_table.clear();
        self.former_transposition_table.clear();

        let valid_moves = state.board.valid_moves(state.current_player);
        if valid_moves.is_empty() {
            return None;
        }

        let mut best_move = None;
        let start_depth = cmp::max(1, self.max_depth - 3);
        for depth in start_depth..=self.max_depth {
            let mut alpha = i32::MIN + 1;
            let beta = i32::MAX;
            let mut children: Vec<(Position, SearchState, i32)> = Vec::new();
            for pos in &valid_moves {
                if let Some(child_state) = apply_move_state(&state, *pos) {
                    let ordering = self.calc_move_ordering_value(&child_state);
                    children.push((*pos, child_state, ordering));
                }
            }
            if children.len() >= 2 {
                children.sort_by(|a, b| b.2.cmp(&a.2));
            }
            for (pos, child_state, _) in children.iter() {
                let score =
                    -self.nega_alpha_transpose(*child_state, depth - 1, false, -beta, -alpha);
                if score > alpha {
                    alpha = score;
                    best_move = Some(*pos);
                }
            }

            // Use the transposition table for move ordering
            std::mem::swap(
                &mut self.transposition_table,
                &mut self.former_transposition_table,
            );
            self.transposition_table.clear();

            // Print for debug
            // println!(
            //     "Depth {}: best_move: {:?}, visited_nodes: {}",
            //     depth, best_move, self.visited_nodes
            // );
        }
        best_move
    }
}

impl<E> Strategy for NegaAlphaTTStrategy<E>
where
    E: EvaluationFunction + Send + Sync + Clone + 'static,
{
    /// Evaluates the game state and decides the next move.
    fn evaluate_and_decide(&mut self, game: &Game) -> Option<Position> {
        let state = SearchState::new(game.board_state().clone(), game.current_player());
        self.search(state)
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new((*self).clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::{evaluator::PhaseAwareEvaluator, strategy::NegaAlphaStrategy};

    use super::*;

    #[test]
    fn test_visited_nodes() {
        let game = Game::default();
        let evaluator = PhaseAwareEvaluator::default();
        let mut strategy = NegaAlphaStrategy::new(evaluator, 10);

        let start = std::time::Instant::now();
        strategy.evaluate_and_decide(&game);
        let elapsed = start.elapsed();
        println!("[NegaAlpha] Elapsed: {:?}", elapsed);
        assert!(
            strategy.nodes_searched > 0,
            "Nodes searched should be greater than 0."
        );
        println!("[NegaAlpha] Visited nodes: {}", strategy.nodes_searched);

        let game = Game::default();
        let evaluator = PhaseAwareEvaluator::default();
        let mut strategy = NegaAlphaTTStrategy::new(evaluator, 10, 0.0);

        let start = std::time::Instant::now();
        strategy.evaluate_and_decide(&game);
        let elapsed = start.elapsed();
        println!("[NegaAlphaTT] Elapsed: {:?}", elapsed);
        assert!(
            strategy.visited_nodes > 0,
            "Visited nodes should be greater than 0."
        );
        println!("[NegaAlphaTT] Visited nodes: {}", strategy.visited_nodes);
    }

    #[test]
    fn test_self_play() {
        let mut game = Game::default();
        let evaluator = PhaseAwareEvaluator::default();
        let mut strategy = NegaAlphaTTStrategy::new(evaluator, 6, 0.0);

        let start = std::time::Instant::now();
        while !game.is_game_over() {
            if let Some(chosen_move) = strategy.evaluate_and_decide(&game) {
                game.apply_move(chosen_move).unwrap();
            } else {
                break;
            }
        }
        let elapsed = start.elapsed();

        println!("[NegaAlphaTT] Elapsed: {:?}", elapsed);

        let mut game = Game::default();
        let evaluator = PhaseAwareEvaluator::default();
        let mut strategy = NegaAlphaStrategy::new(evaluator, 6);

        let start = std::time::Instant::now();
        while !game.is_game_over() {
            if let Some(chosen_move) = strategy.evaluate_and_decide(&game) {
                game.apply_move(chosen_move).unwrap();
            } else {
                break;
            }
        }
        let elapsed = start.elapsed();

        println!("[NegaAlpha] Elapsed: {:?}", elapsed);
    }
}
