pub mod mobility_evaluate;
pub mod negamax;
pub mod positional_evaluate;
pub mod simple_evaluate;

use reversi_core::{board::Board, Color, Move};

pub struct GameState<B: Board> {
    board: B,
    player: Color,
}

impl<B: Board> GameState<B> {
    fn new(board: B, player: Color) -> Self {
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
