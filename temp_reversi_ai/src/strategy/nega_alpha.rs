use crate::evaluator::Evaluator;
use rand::{rng, seq::SliceRandom};
use temp_reversi_core::{Bitboard, Player, Position};

use super::Strategy;

/// The Negamax strategy with alpha-beta pruning.
#[derive(Clone)]
pub struct NegaAlphaStrategy<E>
where
    E: Evaluator + Send + Sync,
{
    pub depth: i32,          // The depth to search in the game tree.
    pub evaluator: E,        // The evaluation function to use.
    pub nodes_searched: u64, // The number of nodes searched in the game tree.
}

impl<E> NegaAlphaStrategy<E>
where
    E: Evaluator + Send + Sync,
{
    /// Creates a new NegamaxStrategy.
    ///
    /// # Arguments
    /// * `evaluator` - The evaluation function to score board states.
    /// * `depth` - The maximum depth of the search tree.
    pub fn new(evaluator: E, depth: i32) -> Self {
        Self {
            depth,
            evaluator,
            nodes_searched: 0,
        }
    }

    /// Negamax recursive function with alpha-beta pruning.
    ///
    /// # Arguments
    /// * `board` - Current state of the board.
    /// * `depth` - Remaining depth to search.
    /// * `alpha` - Current best score for the maximizing player.
    /// * `beta` - Current best score for the minimizing player.
    /// * `player` - The current player making the move.
    ///
    /// # Returns
    /// * `i32` - The score of the board.
    ///
    /// This function shuffles the valid moves to add stochasticity, which helps
    /// avoid deterministic behavior in symmetrical board states.
    fn negamax(
        &mut self,
        board: &Bitboard,
        depth: i32,
        mut alpha: i32,
        beta: i32,
        player: Player,
    ) -> i32 {
        self.nodes_searched += 1;

        // Base case: Leaf node or depth limit reached
        if depth == 0 || board.is_game_over() {
            let score = self.evaluator.evaluate(board, player);
            return score;
        }

        let mut max_eval = std::i32::MIN + 1;
        let mut valid_moves = board.valid_moves(player);

        // Shuffle the moves to introduce randomness
        valid_moves.shuffle(&mut rng());

        for mv in valid_moves {
            let mut new_board = board.clone();
            let r = new_board.apply_move(mv, player);
            if let Err(_) = r {
                panic!();
            }
            let eval = -self.negamax(&new_board, depth - 1, -beta, -alpha, player.opponent());
            max_eval = max_eval.max(eval);
            alpha = alpha.max(eval);
            if alpha >= beta {
                break; // Beta cutoff
            }
        }
        max_eval
    }
}

impl<E> Strategy for NegaAlphaStrategy<E>
where
    E: Evaluator + Clone + Send + Sync + 'static,
{
    /// Evaluates the game state and selects the best move using the Negamax algorithm.
    ///
    /// # Arguments
    /// * `game` - The current game state.
    ///
    /// # Returns
    /// * `Option<Position>` - The position of the selected move or `None` if no valid move exists.
    ///
    /// This method ensures randomness in decision-making by shuffling valid moves.
    fn evaluate_and_decide(&mut self, board: &Bitboard, player: Player) -> Option<Position> {
        self.nodes_searched = 0;

        let mut best_move = None;
        let mut best_score = std::i32::MIN + 1;
        let mut alpha = std::i32::MIN + 1;
        let beta = std::i32::MAX;

        let mut valid_moves = board.valid_moves(player);
        valid_moves.shuffle(&mut rng()); // Shuffle moves for variability

        for &mv in &valid_moves {
            let mut new_board = board.clone();
            new_board.apply_move(mv, player).unwrap();
            let score = -self.negamax(&new_board, self.depth - 1, -beta, -alpha, player.opponent());
            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }
            alpha = alpha.max(score);
        }

        if best_move.is_none() && !valid_moves.is_empty() {
            best_move = Some(valid_moves.first().unwrap().clone());
        }

        best_move
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new((*self).clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluator::{PhaseAwareEvaluator, SimpleEvaluator};

    use super::*;
    use temp_reversi_core::Game;

    #[test]
    fn test_negamax_with_alpha_beta() {
        let game = Game::default();
        let evaluator = SimpleEvaluator;
        let mut strategy = NegaAlphaStrategy::new(evaluator, 1);

        let move_option = strategy.evaluate_and_decide(&game.board_state(), game.current_player());
        assert!(
            move_option.is_some(),
            "NegamaxStrategy with alpha-beta pruning should return a valid move."
        );
    }

    #[test]
    fn test_nodes_searched() {
        let game = Game::default();
        let evaluator = PhaseAwareEvaluator::default();
        let mut strategy = NegaAlphaStrategy::new(evaluator, 9);

        strategy.evaluate_and_decide(&game.board_state(), game.current_player());
        assert!(
            strategy.nodes_searched > 0,
            "Nodes searched should be greater than 0."
        );

        println!("Nodes searched: {}", strategy.nodes_searched);
    }
}
