use crate::{game::GameState, Position};

pub trait Player {
    fn get_move(&mut self, state: &GameState) -> Option<Position>;
}
