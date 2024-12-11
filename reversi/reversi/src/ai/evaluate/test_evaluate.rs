use crate::{board::Board, CellState, Color};

use super::{mobility_evaluate, simple_evaluate};

pub fn test_evaluate<B: Board>(board: &B, color: Color) -> i32 {
    let empty_count = board.count_of(CellState::Empty);
    if empty_count > 10 {
        mobility_evaluate(board, color)
    } else {
        simple_evaluate(board, color)
    }
}
