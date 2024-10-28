pub mod minmax;

use reversi_core::{board::Board, Color, Move};

struct GameState<B: Board> {
    board: B,
    player: Color,
}

impl<B: Board> GameState<B> {
    fn new(board: B, player: Color) -> Self {
        GameState { board, player }
    }
}

struct SearchResult {
    best_move: Option<Move>,
    path: Vec<Move>,
    nodes_searched: usize,
}
