use temp_reversi_core::{Board, Player};

pub trait EvaluationFunction<B: Board> {
    /// Evaluate the current board state for a specific player.
    ///
    /// # Arguments
    /// * `board` - The current board state.
    /// * `player` - The player for whom the evaluation is performed.
    ///
    /// # Returns
    /// * `i32` - The evaluation score.
    fn evaluate(&self, board: &B, player: Player) -> i32;
}

mod mobility;
mod pattern;
mod phase_aware;
mod positional;
mod simple;
mod tempura;

pub use mobility::*;
pub use pattern::*;
pub use phase_aware::*;
pub use positional::*;
pub use simple::*;
pub use tempura::*;
