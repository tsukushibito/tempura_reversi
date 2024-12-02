use crate::{board::Board, Color};

pub fn mobility_evaluate<B: Board>(board: &B, color: Color) -> i32 {
    let my_moves = board.get_valid_moves(color).len() as i32;
    let opponent_moves = board.get_valid_moves(color.opponent()).len() as i32;
    my_moves - opponent_moves
}
