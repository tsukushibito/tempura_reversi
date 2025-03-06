mod nega_alpha;
mod nega_alpha_tt;
mod nega_scout;
mod random;
mod search_state;
mod simple;

use temp_reversi_core::{Game, Position};

/// The `Strategy` trait defines the interface for different strategies.
pub trait Strategy: Send + Sync {
    /// Evaluate the current game state and decide the next move.
    ///
    /// # Arguments
    /// * `game` - The current state of the game.
    ///
    /// # Returns
    /// * `Option<Position>` - The chosen position or `None` if no move is possible.
    fn evaluate_and_decide(&mut self, game: &Game) -> Option<Position>;

    /// Clones the strategy as a `Box<dyn Strategy>`.
    fn clone_box(&self) -> Box<dyn Strategy>;
}

/// Implements `Clone` for `Box<dyn Strategy>` to enable safe cloning.
impl Clone for Box<dyn Strategy> {
    fn clone(&self) -> Box<dyn Strategy> {
        self.clone_box()
    }
}

pub use nega_alpha::*;
pub use nega_alpha_tt::*;
pub use random::*;
pub use simple::*;
