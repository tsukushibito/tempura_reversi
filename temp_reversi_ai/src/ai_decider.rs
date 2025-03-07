use temp_reversi_core::{Board, Game, MoveDecider, Position};

use crate::strategy::Strategy;

/// AI decision-making class that wraps a strategy for move selection.
pub struct AiDecider<B: Board> {
    strategy: Box<dyn Strategy<B>>, // Dynamically chosen strategy
}

impl<B: Board> AiDecider<B> {
    /// Creates a new AI decision maker with a given strategy.
    ///
    /// # Arguments
    /// * `strategy` - The strategy to use for move selection.
    pub fn new(strategy: Box<dyn Strategy<B>>) -> Self {
        Self { strategy }
    }
}

impl<B: Board> MoveDecider<B> for AiDecider<B> {
    /// Selects the next move using the encapsulated strategy.
    ///
    /// # Arguments
    /// * `game` - The current state of the game.
    ///
    /// # Returns
    /// * `Option<Position>` - The chosen move, or `None` if no move is possible.
    fn select_move(&mut self, game: &Game<B>) -> Option<Position> {
        self.strategy.evaluate_and_decide(game)
    }
}
