use temp_reversi_core::{Game, GamePlayer, Position};

use crate::strategy::Strategy;

/// AI decision-making class that wraps a strategy for move selection.
pub struct AiPlayer {
    strategy: Box<dyn Strategy>, // Dynamically chosen strategy
}

impl AiPlayer {
    /// Creates a new AI decision maker with a given strategy.
    ///
    /// # Arguments
    /// * `strategy` - The strategy to use for move selection.
    pub fn new(strategy: Box<dyn Strategy>) -> Self {
        Self { strategy }
    }
}

impl GamePlayer for AiPlayer {
    fn select_move(&mut self, game: &Game) -> Position {
        self.strategy
            .select_move(game.board_state(), game.current_player())
    }
}
