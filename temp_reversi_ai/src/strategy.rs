mod nega_alpha;
mod nega_alpha_tt;
mod nega_scout;
mod nega_scout2;
mod random;
mod search_state;
mod simple;

use temp_reversi_core::{Board, Game, Position};

/// The `Strategy` trait defines the interface for different strategies.
pub trait Strategy<B: Board>: Send + Sync {
    /// Evaluate the current game state and decide the next move.
    ///
    /// # Arguments
    /// * `game` - The current state of the game.
    ///
    /// # Returns
    /// * `Option<Position>` - The chosen position or `None` if no move is possible.
    fn evaluate_and_decide(&mut self, game: &Game<B>) -> Option<Position>;

    /// Clones the strategy as a `Box<dyn Strategy>`.
    fn clone_box(&self) -> Box<dyn Strategy<B>>;
}

/// Implements `Clone` for `Box<dyn Strategy>` to enable safe cloning.
impl<B: Board> Clone for Box<dyn Strategy<B>> {
    fn clone(&self) -> Box<dyn Strategy<B>> {
        self.clone_box()
    }
}

pub use nega_alpha::*;
pub use nega_alpha_tt::*;
pub use nega_scout::*;
pub use nega_scout2::*;
pub use random::*;
pub use simple::*;
