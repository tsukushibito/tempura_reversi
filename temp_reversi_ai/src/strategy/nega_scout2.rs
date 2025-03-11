use std::hash::Hash;

use temp_game_ai::search::{Evaluator, GameState, NegaScout};
use temp_reversi_core::{Bitboard, Board, Player, Position};

use crate::evaluator::{EvaluationFunction, TempuraEvaluator};

use super::Strategy;

#[derive(Clone, PartialEq, Eq)]
struct ReversiState {
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

    fn generate_children(&self) -> Vec<Self> {
        todo!()
    }
}

struct ReversiEvaluator {
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
        self.evaluator.evaluate(&state.board, state.player)
    }
}

pub struct NegaScoutStrategy2 {
    nega_scout: NegaScout<ReversiState, ReversiEvaluator>,
    max_depth: i32,
}

impl NegaScoutStrategy2 {
    pub fn new(model_path: &str, max_depth: i32) -> Self {
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

        // self.nega_scout.iterative_deepening(root, self.max_depth)
        None
    }

    fn clone_box(&self) -> Box<dyn Strategy<B>> {
        todo!()
    }
}
