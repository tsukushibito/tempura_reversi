use std::hash::Hash;

use temp_game_ai::search::{Evaluator, GameState, NegaScout};
use temp_reversi_core::{Bitboard, Board, Player, Position};

use crate::evaluator::{
    EvaluationFunction, MobilityEvaluator, PhaseAwareEvaluator, TempuraEvaluator,
};

use super::Strategy;

#[derive(Clone, PartialEq, Eq)]
pub struct ReversiState {
    board: Bitboard,
    player: Player,
}

impl Hash for ReversiState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let (black, white) = self.board.bits();
        black.hash(state);
        white.hash(state);
        self.player.hash(state);
    }
}

impl GameState for ReversiState {
    type Move = Position;

    fn is_terminal(&self) -> bool {
        self.board.valid_moves(self.player).is_empty()
            && self.board.valid_moves(self.player.opponent()).is_empty()
    }

    fn generate_children(&self) -> Vec<(Self, Self::Move)> {
        self.board
            .valid_moves(self.player)
            .iter()
            .map(|&pos| {
                let mut board = self.board.clone();
                board.apply_move(pos, self.player).unwrap();
                (
                    ReversiState {
                        board,
                        player: self.player.opponent(),
                    },
                    pos,
                )
            })
            .collect()
    }
}

pub struct ReversiEvaluator {
    evaluator: TempuraEvaluator,
}

impl ReversiEvaluator {
    fn new(model_path: &str) -> Self {
        Self {
            evaluator: TempuraEvaluator::new(model_path),
        }
    }
}

impl Evaluator<ReversiState> for ReversiEvaluator {
    fn evaluate(&self, state: &ReversiState) -> i32 {
        self.evaluator.evaluate(&state.board, state.player)
    }

    fn order_evaluate(&self, state: &ReversiState) -> i32 {
        PhaseAwareEvaluator::default().evaluate(&state.board, state.player)
        // self.evaluator.evaluate(&state.board, state.player)
    }
}

pub struct NegaScoutStrategy2 {
    pub nega_scout: NegaScout<ReversiState, ReversiEvaluator>,
    max_depth: usize,
}

impl NegaScoutStrategy2 {
    pub fn new(model_path: &str, max_depth: usize) -> Self {
        let evaluator = ReversiEvaluator::new(model_path);
        let nega_scout = NegaScout::new(evaluator);
        Self {
            nega_scout,
            max_depth,
        }
    }
}

impl<B> Strategy<B> for NegaScoutStrategy2
where
    B: Board,
{
    fn evaluate_and_decide(
        &mut self,
        game: &temp_reversi_core::Game<B>,
    ) -> Option<temp_reversi_core::Position> {
        let (black, white) = game.board_state().bits();
        let bitboard = Bitboard::new(black, white);
        let root = ReversiState {
            board: bitboard,
            player: game.current_player(),
        };

        let best_move = self.nega_scout.search_best_move(&root, self.max_depth);
        best_move
    }

    fn clone_box(&self) -> Box<dyn Strategy<B>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use temp_reversi_core::{Bitboard, Game};

    use crate::strategy::{NegaAlphaTTStrategy, NegaScoutStrategy};

    use super::*;

    #[test]
    fn test_visited_nodes() {
        let depth = 9;

        // let game = Game::default();
        // let evaluator = PhaseAwareEvaluator::default();
        // let mut strategy = NegaAlphaStrategy::new(evaluator, depth);

        // let start = std::time::Instant::now();
        // strategy.evaluate_and_decide(&game);
        // let elapsed = start.elapsed();
        // println!("[NegaAlpha] Elapsed: {:?}", elapsed);
        // assert!(
        //     strategy.nodes_searched > 0,
        //     "Nodes searched should be greater than 0."
        // );
        // println!("[NegaAlpha] Visited nodes: {}", strategy.nodes_searched);

        let mut game = Game::<Bitboard>::default();
        let valid_moves = game.valid_moves();
        game.apply_move(valid_moves[0]).unwrap();
        let valid_moves = game.valid_moves();
        game.apply_move(valid_moves[0]).unwrap();
        let evaluator = TempuraEvaluator::new("../gen0/models/temp_model.bin");
        let mut strategy = NegaAlphaTTStrategy::new(evaluator, depth, 0.0);

        let start = std::time::Instant::now();
        strategy.evaluate_and_decide(&game);
        let elapsed = start.elapsed();
        println!("[NegaAlphaTT] Elapsed: {:?}", elapsed);
        assert!(
            strategy.visited_nodes > 0,
            "Visited nodes should be greater than 0."
        );
        println!("[NegaAlphaTT] Visited nodes: {}", strategy.visited_nodes);

        let mut game = Game::<Bitboard>::default();
        let valid_moves = game.valid_moves();
        game.apply_move(valid_moves[0]).unwrap();
        let valid_moves = game.valid_moves();
        game.apply_move(valid_moves[0]).unwrap();
        let evaluator = TempuraEvaluator::new("../gen0/models/temp_model.bin");
        let mut strategy = NegaScoutStrategy::new(evaluator, depth, 0.0);

        let start = std::time::Instant::now();
        strategy.evaluate_and_decide(&game);
        let elapsed = start.elapsed();
        println!("[NegaScout] Elapsed: {:?}", elapsed);
        assert!(
            strategy.visited_nodes > 0,
            "Visited nodes should be greater than 0."
        );
        println!("[NegaScout] Visited nodes: {}", strategy.visited_nodes);

        let mut game = Game::<Bitboard>::default();
        let valid_moves = game.valid_moves();
        game.apply_move(valid_moves[0]).unwrap();
        let valid_moves = game.valid_moves();
        game.apply_move(valid_moves[0]).unwrap();
        let mut strategy = NegaScoutStrategy2::new("../gen0/models/temp_model.bin", depth as usize);

        let start = std::time::Instant::now();
        strategy.evaluate_and_decide(&game);
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
}
