use crate::{bit_board::BitBoard, Color, Position};

use super::{bit_pattern::BitPattern, Evaluator};

struct PatternEvalTable {
    pub pattern_scores: Vec<Vec<f32>>,
}

impl PatternEvalTable {
    pub fn new(patterns: &[BitPattern]) -> Self {
        let mut pattern_scores = Vec::new();
        for p in patterns {
            let length = p.pattern_length();
            let num_states = 3usize.pow(length as u32);
            pattern_scores.push(vec![0.0f32; num_states]);
        }
        PatternEvalTable { pattern_scores }
    }

    pub fn get_score(&self, pattern_id: usize, state_idx: usize) -> f32 {
        self.pattern_scores[pattern_id][state_idx]
    }

    pub fn set_score(&mut self, pattern_id: usize, state_idx: usize, score: f32) {
        self.pattern_scores[pattern_id][state_idx] = score;
    }
}

pub struct PatternEvaluator {
    patterns: Vec<BitPattern>,
    eval_table: PatternEvalTable,
}

impl Evaluator for PatternEvaluator {
    fn evaluate(&self, board: &BitBoard, color: Color) -> i32 {
        let mut total_score = 0.0f32;

        for pattern in &self.patterns {
            let state_idx = pattern.pattern_state_index(board);
            let score = self.eval_table.get_score(pattern.id, state_idx);
            total_score += score * pattern.weight;
        }

        match color {
            Color::Black => total_score as i32,
            Color::White => -total_score as i32,
        }
    }
}
