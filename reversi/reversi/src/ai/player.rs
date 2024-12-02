use crate::{game_play::GameState, Position};

pub trait Player {
    fn get_move(&mut self, state: &GameState) -> Option<Position>;
}
