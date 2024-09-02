use temp_reversi_ai::{
    evaluation::PhaseAwareEvaluator,
    strategy::{negamax::NegamaxStrategy, Strategy},
};
use temp_reversi_cli::{cli_display, CliPlayer};
use temp_reversi_core::{run_game, Game, MoveDecider, Position};

/// A wrapper to use NegamaxStrategy with MoveDecider trait.
struct NegamaxMoveDecider {
    strategy: NegamaxStrategy<PhaseAwareEvaluator>,
}

impl NegamaxMoveDecider {
    pub fn new(depth: u32) -> Self {
        let evaluator = PhaseAwareEvaluator;
        let strategy = NegamaxStrategy::new(evaluator, depth);
        Self { strategy }
    }
}

impl MoveDecider for NegamaxMoveDecider {
    fn select_move(&mut self, game: &Game) -> Option<Position> {
        self.strategy.evaluate_and_decide(game)
    }
}

/// Entry point for the CLI-based Reversi game.
fn main() -> Result<(), String> {
    let ai_player = NegamaxMoveDecider::new(5); // Depth of 3 for Black
    run_game(ai_player, CliPlayer {}, cli_display)
}
