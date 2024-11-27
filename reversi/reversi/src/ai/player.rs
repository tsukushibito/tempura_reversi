use crate::{board::Board, game_play::GameState, Position};

pub trait Player<B: Board> {
    fn get_move(&mut self, state: &GameState<B>) -> Option<Position>;
}
