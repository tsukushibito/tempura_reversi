use crate::{board::Board, Position};

use super::GameState;

pub trait Player<B: Board> {
    fn get_move(&mut self, state: &GameState<B>) -> Option<Position>;
}
