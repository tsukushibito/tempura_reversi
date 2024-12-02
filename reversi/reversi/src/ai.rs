use crate::{board::BOARD_SIZE, Move};

pub mod ai_player;
pub mod evaluate;
pub mod human_player;
pub mod player;
pub mod search;

pub struct SearchResult {
    pub best_move: Option<Move>,
    pub path: Vec<Move>,
    pub nodes_searched: usize,
    pub score: i32,
    pub policy: [i32; BOARD_SIZE * BOARD_SIZE],
}
