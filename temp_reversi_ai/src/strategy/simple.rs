use super::Strategy;
use temp_reversi_core::{Bitboard, Player, Position};

/// A simple strategy that selects the first valid move.
#[derive(Clone, Debug)]
pub struct SimpleStrategy;

impl Strategy for SimpleStrategy {
    fn select_move(&mut self, board: &Bitboard, player: Player) -> Position {
        board.valid_moves(player).into_iter().next().unwrap()
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
