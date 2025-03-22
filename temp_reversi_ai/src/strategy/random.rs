use super::Strategy;
use rand::{prelude::*, rng};
use temp_reversi_core::{Bitboard, Player, Position};

/// A random strategy that selects a move randomly from the list of valid moves.
#[derive(Clone, Debug)]
pub struct RandomStrategy;

impl Strategy for RandomStrategy {
    fn select_move(&mut self, board: &Bitboard, player: Player) -> Position {
        let mut rng = rng();
        let valid_moves = board.valid_moves(player);
        *valid_moves.choose(&mut rng).unwrap()
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
