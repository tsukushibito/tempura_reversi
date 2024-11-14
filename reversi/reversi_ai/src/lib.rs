pub mod evaluate;
pub mod search;

use reversi_core::{board::Board, Color, Move};

pub struct GameState<B: Board> {
    board: B,
    player: Color,
}

impl<B: Board> GameState<B> {
    pub fn new(board: B, player: Color) -> Self {
        GameState { board, player }
    }
}

pub struct SearchResult {
    best_move: Option<Move>,
    path: Vec<Move>,
    nodes_searched: usize,
    score: i32,
}

pub type EvalFunc<B> = fn(&GameState<B>, Color) -> i32;
