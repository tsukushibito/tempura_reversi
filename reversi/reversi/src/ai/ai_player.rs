use crate::{bit_board::BitBoard, Color, Position};

use super::{player::Player, search::Negaalpha};

pub struct AiPlayer {
    searcher: Negaalpha,
    color: Color,
    // 必要に応じて他のフィールドを追加
}

impl AiPlayer {
    pub fn new(evaluate_fn: fn(&BitBoard, Color) -> i32, color: Color) -> Self {
        AiPlayer {
            searcher: Negaalpha::new(evaluate_fn),
            color,
        }
    }
}

impl Player for AiPlayer {
    fn get_move(&mut self, board: &BitBoard, color: Color) -> Option<Position> {
        let search_result = self
            .searcher
            .search(board, color, 8, i32::MIN + 1, i32::MAX);
        search_result.best_move.map(|mv| mv.position)
    }
}
