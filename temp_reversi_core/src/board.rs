use crate::{Player, Position};

pub trait Board: Clone + Default + Eq + PartialEq + std::fmt::Display {
    fn bits(&self) -> (u64, u64);
    fn valid_moves(&self, player: Player) -> Vec<Position>;
    fn count_stones(&self) -> (usize, usize);
    fn is_game_over(&self) -> bool;
    fn apply_move(&mut self, position: Position, player: Player) -> Result<(), &'static str>;
    fn get_hash(&self) -> u64;
}
