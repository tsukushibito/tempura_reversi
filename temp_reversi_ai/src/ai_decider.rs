use temp_reversi_core::{Game, MoveDecider, Position};

use crate::strategy::Strategy;

/// AI decision-making class that wraps a strategy for move selection.
pub struct AiDecider {
    strategy: Box<dyn Strategy>, // Dynamically chosen strategy
}

impl AiDecider {
    /// Creates a new AI decision maker with a given strategy.
    ///
    /// # Arguments
    /// * `strategy` - The strategy to use for move selection.
    pub fn new(strategy: Box<dyn Strategy>) -> Self {
        Self { strategy }
    }
}

impl MoveDecider for AiDecider {
    /// Selects the next move using the encapsulated strategy.
    ///
    /// # Arguments
    /// * `game` - The current state of the game.
    ///
    /// # Returns
    /// * `Option<Position>` - The chosen move, or `None` if no move is possible.
    fn select_move(&mut self, game: &Game) -> Option<Position> {
        self.strategy
            .evaluate_and_decide(game.board_state(), game.current_player())
    }
}
