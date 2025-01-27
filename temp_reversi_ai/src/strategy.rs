pub mod negamax;
pub mod random;
pub mod simple;

use temp_reversi_core::{Game, Position};

/// The `Strategy` trait defines the interface for different strategies.
pub trait Strategy {
    /// Evaluate the current game state and decide the next move.
    ///
    /// # Arguments
    /// * `game` - The current state of the game.
    ///
    /// # Returns
    /// * `Option<Position>` - The chosen position or `None` if no move is possible.
    fn evaluate_and_decide(&mut self, game: &Game) -> Option<Position>;
}
