use crate::{bit_board::BitBoard, Color, Position};

pub trait Player {
    fn get_move(&mut self, board: &BitBoard, color: Color) -> Option<Position>;
}
