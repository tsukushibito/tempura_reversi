use crate::{board::Board, board::BOARD_SIZE, Color, Move};

pub mod ai_player;
pub mod evaluate;
pub mod game_play;
pub mod human_player;
pub mod player;
pub mod search;

pub struct GameState<B: Board> {
    pub board: B,
    pub player: Color,
}

impl<B: Board> GameState<B> {
    pub fn new(board: B, player: Color) -> Self {
        GameState { board, player }
    }
}

pub struct SearchResult {
    pub best_move: Option<Move>,
    pub path: Vec<Move>,
    pub nodes_searched: usize,
    pub score: i32,
    pub policy: [i32; BOARD_SIZE * BOARD_SIZE],
}

pub type EvalFunc<B> = fn(&GameState<B>, Color) -> i32;
