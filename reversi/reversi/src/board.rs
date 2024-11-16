use crate::{Color, Position};
use std::hash::Hash;

pub const BOARD_SIZE: usize = 8;

pub trait Board: Clone + Default + Hash + PartialEq + Eq {
    fn new() -> Self {
        let mut board = Self::default();
        board.set_disc(&Position::D4, Some(Color::White));
        board.set_disc(&Position::E5, Some(Color::White));
        board.set_disc(&Position::D5, Some(Color::Black));
        board.set_disc(&Position::E4, Some(Color::Black));

        board
    }

    fn discs(&self) -> Vec<Vec<Option<Color>>>;
    fn get_disc(&self, pos: &Position) -> Option<Color>;
    fn set_disc(&mut self, pos: &Position, color: Option<Color>);

    fn count_of(&self, color: Option<Color>) -> usize;

    fn black_count(&self) -> usize {
        self.count_of(Some(Color::Black))
    }

    fn white_count(&self) -> usize {
        self.count_of(Some(Color::White))
    }

    fn empty_count(&self) -> usize {
        self.count_of(None)
    }

    fn make_move(&mut self, color: Color, pos: &Position) -> bool;

    fn get_valid_moves(&self, color: Color) -> Vec<Position>;

    fn display(&self);
}
