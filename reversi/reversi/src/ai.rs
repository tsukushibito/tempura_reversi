use crate::{bit_board::BitBoard, board::BOARD_SIZE, Color, Move, Position};

mod evaluator;
mod pattern;
mod search;

pub use evaluator::*;
pub use pattern::*;
pub use search::*;

pub struct SearchResult {
    pub best_move: Option<Move>,
    pub path: Vec<Move>,
    pub nodes_searched: usize,
    pub score: i32,
    pub policy: [i32; BOARD_SIZE * BOARD_SIZE],
}

pub enum Searcher {
    TestNegaalpha(Negaalpha<TestEvaluator>),
    TempuraNegaalpha(Negaalpha<TempuraEvaluator>),
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
            Searcher::TempuraNegaalpha(s) => s.search(board, player, depth, alpha, beta),
        }
    }
}

pub struct Ai {
    pub searcher: Searcher,
    pub search_depth: u8,
}

impl Default for Ai {
    fn default() -> Self {
        let searcher = Searcher::TestNegaalpha(Negaalpha::new(TestEvaluator::default()));
        Self {
            searcher,
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
