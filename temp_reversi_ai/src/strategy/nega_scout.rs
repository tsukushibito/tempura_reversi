use temp_game_ai::searcher::{NegaScout, Searcher};
use temp_reversi_core::{Bitboard, Player};

use crate::evaluator::{PhaseAwareEvaluator, ReversiState, TempuraEvaluator};

use super::Strategy;

#[derive(Clone, Debug)]
pub struct NegaScoutStrategy {
    pub nega_scout: NegaScout<ReversiState, TempuraEvaluator, PhaseAwareEvaluator>,
    max_depth: usize,
}

impl NegaScoutStrategy {
    pub fn new(evaluator: TempuraEvaluator, max_depth: usize) -> Self {
        let order_evaluator = PhaseAwareEvaluator::default();
        let nega_scout = NegaScout::new(evaluator, order_evaluator);
        Self {
            nega_scout,
            max_depth,
        }
    }
}

impl Strategy for NegaScoutStrategy {
    fn select_move(
        &mut self,
        board: &Bitboard,
        player: Player,
    ) -> Option<temp_reversi_core::Position> {
        let root = ReversiState {
            board: *board,
            player,
        };

        if let Some(best_move) = self.nega_scout.search(&root, self.max_depth) {
            Some(best_move.0)
        } else {
            None
        }
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use temp_reversi_core::Game;

    use crate::strategy::NegaAlphaTTStrategy;

    use super::*;

    #[test]
    fn test_visited_nodes() {
        let depth = 9;

        let mut game = Game::default();
        let valid_moves = game.valid_moves();
        game.apply_move(valid_moves[0]).unwrap();
        let valid_moves = game.valid_moves();
        game.apply_move(valid_moves[0]).unwrap();
        let evaluator = TempuraEvaluator::new("../gen0/models/temp_model.bin");
        let mut strategy = NegaAlphaTTStrategy::new(evaluator, depth);

        let start = std::time::Instant::now();
        strategy.select_move(&game.board_state(), game.current_player());
        let elapsed = start.elapsed();
        println!("[NegaAlphaTT] Elapsed: {:?}", elapsed);
        assert!(
            strategy.nega_alpha_tt.visited_nodes > 0,
            "Visited nodes should be greater than 0."
        );
        println!(
            "[NegaAlphaTT] Visited nodes: {}",
            strategy.nega_alpha_tt.visited_nodes
        );

        let mut game = Game::default();
        let valid_moves = game.valid_moves();
        game.apply_move(valid_moves[0]).unwrap();
        let valid_moves = game.valid_moves();
        game.apply_move(valid_moves[0]).unwrap();
        let evaluator = TempuraEvaluator::new("../gen0/models/temp_model.bin");
        let mut strategy = NegaScoutStrategy::new(evaluator, depth as usize);

        let start = std::time::Instant::now();
        strategy.select_move(&game.board_state(), game.current_player());
        let elapsed = start.elapsed();
        println!("[NegaScout2] Elapsed: {:?}", elapsed);
        assert!(
            strategy.nega_scout.visited_nodes > 0,
            "Visited nodes should be greater than 0."
        );
        println!(
            "[NegaScout2] Visited nodes: {}",
            strategy.nega_scout.visited_nodes
        );
    }

    #[test]
    fn test_self_play() {
        let depth = 5;

        let mut game = Game::default();
        let evaluator = TempuraEvaluator::new("../gen0/models/temp_model.bin");
        let mut strategy1 = NegaScoutStrategy::new(evaluator, depth as usize);

        let start = std::time::Instant::now();
        while !game.is_game_over() {
            let best_move = strategy1.select_move(&game.board_state(), game.current_player());
            if let Some(best_move) = best_move {
                game.apply_move(best_move).unwrap();
            } else {
                break;
            }
        }
        let elapsed = start.elapsed();
        println!("[NegaScout2] Elapsed: {:?}", elapsed);
    }
}
