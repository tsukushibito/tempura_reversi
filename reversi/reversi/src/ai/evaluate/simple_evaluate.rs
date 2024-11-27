use crate::{board::Board, game_play::GameState, Color};

pub fn simple_evaluate<B: Board>(state: &GameState<B>, color: Color) -> i32 {
    let black_count = state.board.black_count() as i32;
    let white_count = state.board.white_count() as i32;
    match color {
        Color::Black => black_count - white_count,
        Color::White => white_count - black_count,
    }
}
