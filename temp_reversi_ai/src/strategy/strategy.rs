use temp_reversi_core::{Bitboard, Player, Position};

/// The `Strategy` trait defines the interface for different strategies.
pub trait Strategy {
    fn select_move(&mut self, board: &Bitboard, player: Player) -> Position;

    /// Clones the strategy as a `Box<dyn Strategy>`.
    fn clone_box(&self) -> Box<dyn Strategy>;
}

/// Implements `Clone` for `Box<dyn Strategy>` to enable safe cloning.
impl Clone for Box<dyn Strategy> {
    fn clone(&self) -> Box<dyn Strategy> {
        self.clone_box()
    }
}
