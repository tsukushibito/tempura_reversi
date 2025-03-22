use temp_game_ai::{
    searcher::{NegaScout, Searcher},
    Evaluator,
};
use temp_reversi_core::{Bitboard, Player, Position};

use super::Strategy;
use crate::ReversiState;

#[derive(Clone, Debug)]
pub struct NegaScoutStrategy<E, O>
where
    E: Evaluator<ReversiState>,
    O: Evaluator<ReversiState>,
{
    pub nega_scout: NegaScout<ReversiState, E, O>,
    max_depth: usize,
}

impl<E, O> NegaScoutStrategy<E, O>
where
    E: Evaluator<ReversiState>,
    O: Evaluator<ReversiState>,
{
    pub fn new(evaluator: E, order_evaluator: O, max_depth: usize) -> Self {
        let nega_scout = NegaScout::new(evaluator, order_evaluator);
        Self {
            nega_scout,
            max_depth,
        }
    }
}

impl<E, O> Strategy for NegaScoutStrategy<E, O>
where
    E: Evaluator<ReversiState> + Clone + 'static,
    O: Evaluator<ReversiState> + Clone + 'static,
{
    fn select_move(&mut self, board: &Bitboard, player: Player) -> Position {
        let root = ReversiState {
            board: *board,
            player,
        };

        self.nega_scout
            .search(&root, self.max_depth)
            .expect("No moves available.")
            .0
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use temp_reversi_core::Game;

    use crate::{
        evaluator::{PhaseAwareEvaluator, TempuraEvaluator},
        strategy::NegaAlphaTTStrategy,
    };

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
        let mut strategy =
            NegaAlphaTTStrategy::new(evaluator, PhaseAwareEvaluator::default(), depth);

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
        let mut strategy =
            NegaScoutStrategy::new(evaluator, PhaseAwareEvaluator::default(), depth as usize);

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
        let mut strategy1 =
            NegaScoutStrategy::new(evaluator, PhaseAwareEvaluator::default(), depth as usize);

        let start = std::time::Instant::now();
        while !game.is_game_over() {
            let best_move = strategy1.select_move(&game.board_state(), game.current_player());
            game.apply_move(best_move).unwrap();
        }
        let elapsed = start.elapsed();
        println!("[NegaScout2] Elapsed: {:?}", elapsed);
    }
}
