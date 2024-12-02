use crate::{board::Board, Color};

pub fn simple_evaluate<B: Board>(board: &B, color: Color) -> i32 {
    let black_count = board.black_count() as i32;
    let white_count = board.white_count() as i32;
    match color {
        Color::Black => black_count - white_count,
        Color::White => white_count - black_count,
    }
}
