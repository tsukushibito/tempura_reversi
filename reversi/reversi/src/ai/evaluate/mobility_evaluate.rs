use crate::{ai::GameState, board::Board, Color};

pub fn mobility_evaluate<B: Board>(state: &GameState<B>, color: Color) -> i32 {
    let my_moves = state.board.get_valid_moves(color).len() as i32;
    let opponent_moves = state.board.get_valid_moves(color.opponent()).len() as i32;
    my_moves - opponent_moves
}
