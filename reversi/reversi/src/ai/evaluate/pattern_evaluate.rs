use crate::{bit_board::BitBoard, Color};

use super::{Evaluator, PatternTable};

pub struct PatternEvaluator {
    pub pattern_table: PatternTable,
}

impl Evaluator for PatternEvaluator {
    fn evaluate(&self, board: &BitBoard, color: Color) -> i32 {
        let score = self.pattern_table.evaluate(board);

        match color {
            Color::Black => score as i32,
            Color::White => -score as i32,
        }
    }
}
