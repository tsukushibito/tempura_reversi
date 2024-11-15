use crate::GameState;
use reversi_core::{board::Board, Position};

pub trait Player<B: Board> {
    fn get_move(&mut self, state: &GameState<B>) -> Option<Position>;
}
