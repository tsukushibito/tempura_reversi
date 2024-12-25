use crate::{bit_board::BitBoard, board::BOARD_SIZE, Color, Move, Position};

mod evaluate;
mod learner;
mod model;
mod pattern;
mod search;
mod self_play;
mod sparse_feature;
mod training;

pub use evaluate::*;
pub use learner::*;
pub use model::*;
pub use pattern::*;
pub use search::*;
pub use self_play::*;
pub use training::*;

pub struct SearchResult {
    pub best_move: Option<Move>,
    pub path: Vec<Move>,
    pub nodes_searched: usize,
    pub score: i32,
    pub policy: [i32; BOARD_SIZE * BOARD_SIZE],
}

pub enum Searcher {
    TestNegaalpha(Negaalpha<TestEvaluator>),
    PatternNegaalpha(Negaalpha<PatternEvaluator>),
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
            Searcher::PatternNegaalpha(s) => s.search(board, player, depth, alpha, beta),
        }
    }
}

pub struct Ai {
    pub searcher: Searcher,
    pub search_depth: u8,
}

impl Default for Ai {
    fn default() -> Self {
        let pattern_table = PatternTable::load("model.bin").unwrap();
        let searcher =
            Searcher::PatternNegaalpha(Negaalpha::new(PatternEvaluator { pattern_table }));
        Self {
            // searcher: Searcher::TestNegaalpha(Negaalpha::new(TestEvaluator::default())),
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
