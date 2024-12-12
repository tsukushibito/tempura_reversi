use crate::{bit_board::BitBoard, board::BOARD_SIZE, Color, Move, Position};

use self::search::Negaalpha;

pub mod evaluate;
pub mod search;
mod self_play;

use evaluate::TestEvaluator;
pub use self_play::*;

pub struct SearchResult {
    pub best_move: Option<Move>,
    pub path: Vec<Move>,
    pub nodes_searched: usize,
    pub score: i32,
    pub policy: [i32; BOARD_SIZE * BOARD_SIZE],
}

enum Searcher {
    TestNegaalpha(Negaalpha<TestEvaluator>),
}

impl Searcher {
    pub fn search(
        &mut self,
        board: &BitBoard,
        player: Color,
        depth: u8,
        alpha: i32,
        beta: i32,
    ) -> SearchResult {
        match self {
            Searcher::TestNegaalpha(s) => s.search(board, player, depth, alpha, beta),
        }
    }
}

pub struct Ai {
    searcher: Searcher,
    search_depth: u8,
}

impl Default for Ai {
    fn default() -> Self {
        Self {
            searcher: Searcher::TestNegaalpha(Negaalpha::new(TestEvaluator::default())),
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
