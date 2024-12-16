use rand::Rng;

use crate::{bit_board::BitBoard, Color};

use super::{bit_pattern::BitPattern, Evaluator};

struct PatternEvalTable {
    pub pattern_scores: Vec<Vec<f32>>,
}

impl PatternEvalTable {
    pub fn new(patterns: &[BitPattern]) -> Self {
        let mut rng = rand::thread_rng();
        let mut pattern_scores = Vec::new();
        for p in patterns {
            let length = p.pattern_length();
            let num_states = 3usize.pow(length as u32);
            let scores: Vec<f32> = (0..num_states)
                .map(|_| rng.gen_range(-64.0..64.0))
                .collect();
            pattern_scores.push(scores);
        }
        PatternEvalTable { pattern_scores }
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
            let score = self.eval_table.pattern_scores[pattern.id][state_idx];
            total_score += score;
        }

        match color {
            Color::Black => total_score as i32,
            Color::White => -total_score as i32,
        }
    }
}
