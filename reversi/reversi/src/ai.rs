use crate::{bit_board::BitBoard, board::BOARD_SIZE, Color, Move, Position};

use self::search::Negaalpha;

pub mod evaluate;
pub mod search;

pub struct SearchResult {
    pub best_move: Option<Move>,
    pub path: Vec<Move>,
    pub nodes_searched: usize,
    pub score: i32,
    pub policy: [i32; BOARD_SIZE * BOARD_SIZE],
}

pub struct Ai {
    searcher: Negaalpha,
    search_depth: u8,
}

impl Default for Ai {
    fn default() -> Self {
        Self {
            searcher: Negaalpha::new(evaluate::test_evaluate),
            search_depth: 8,
        }
    }
}

impl Ai {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn decide_move(&mut self, board: &BitBoard, color: Color) -> Option<Position> {
        let search_result =
            self.searcher
                .search(board, color, self.search_depth, i32::MIN + 1, i32::MAX);
        search_result.best_move.map(|mv| mv.position)
    }
}
