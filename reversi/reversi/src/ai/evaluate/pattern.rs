use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{bit_board::BitBoard, Position};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Pattern {
    pub id: usize,
    pub mask: u64,
}

impl Pattern {
    pub fn from_positions(id: usize, positions: &[Position]) -> Self {
        let mut mask = 0u64;
        for pos in positions {
            let bit_index = pos.to_index();
            mask |= 1 << bit_index;
        }

        Self { id, mask }
    }

    pub fn pattern_length(&self) -> usize {
        self.mask.count_ones() as usize
    }

    pub fn pattern_state_index(&self, board: &BitBoard) -> usize {
        let black_pattern = board.black & self.mask;
        let white_pattern = board.white & self.mask;

        let mut idx = 0;
        let mut mask_copy = self.mask;

        while mask_copy != 0 {
            let bit = mask_copy & (!mask_copy + 1);
            let val = if (black_pattern & bit) != 0 {
                1
            } else if (white_pattern & bit) != 0 {
                2
            } else {
                0
            };

            idx = idx * 3 + val;
            mask_copy &= mask_copy - 1;
        }

        idx
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PatternTable {
    patterns: Vec<Pattern>,
    index_offsets: Vec<usize>,
    scores: Vec<f32>,
}

impl Default for PatternTable {
    fn default() -> Self {
        let patterns = generate_all_line_patterns();
        let mut rng = rand::thread_rng();
        let mut index_offsets = Vec::new();
        let mut index_offset = 0;
        let mut scores = Vec::new();

        patterns.iter().enumerate().for_each(|(id, p)| {
            assert!(id == p.id, "idは連番");

            let length = p.pattern_length();

            index_offsets.push(index_offset);
            index_offset += length;

            let num_states = 3usize.pow(length as u32);
            (0..num_states)
                .map(|_| rng.gen_range(-2.0..2.0))
                .for_each(|v| scores.push(v));
        });

        PatternTable {
            patterns: patterns.to_vec(),
            index_offsets,
            scores,
        }
    }
}

impl PatternTable {
    pub fn new(patterns: &[Pattern], index_offsets: &[usize], scores: &[f32]) -> Self {
        Self {
            patterns: patterns.to_vec(),
            index_offsets: index_offsets.to_vec(),
            scores: scores.to_vec(),
        }
    }

    pub fn patterns(&self) -> &Vec<Pattern> {
        &self.patterns
    }

    pub fn scores(&self) -> &Vec<f32> {
        &self.scores
    }

    pub fn set_scores(&mut self, scores: &[f32]) {
        if scores.len() != self.scores.len() {
            panic!();
        }
        self.scores = scores.to_vec();
    }

    pub fn features(&self, board: &BitBoard) -> Vec<f32> {
        let mut features = vec![0.0; self.scores.len()];

        self.patterns.iter().for_each(|pattern| {
            let score_index = self.score_index(board, pattern);
            features[score_index] = 1.0;
        });

        features
    }

    pub fn evaluate(&self, board: &BitBoard) -> f32 {
        self.patterns
            .iter()
            .map(|pattern| self.scores[self.score_index(board, pattern)])
            .sum()
    }

    fn score_index(&self, board: &BitBoard, pattern: &Pattern) -> usize {
        let state_index = pattern.pattern_state_index(board);
        let index_offset = self.index_offsets[pattern.id];
        index_offset + state_index
    }
}

/// 水平ラインパターン(8行)を生成
fn generate_horizontal_patterns(start_id: usize) -> (Vec<Pattern>, usize) {
    let mut patterns = Vec::new();
    let mut id = start_id;
    for y in 0..8 {
        let positions: Vec<_> = (0..8).map(|x| Position { x, y }).collect();
        patterns.push(Pattern::from_positions(id, &positions));
        id += 1;
    }
    (patterns, id)
}

/// 垂直ラインパターン(8列)を生成
fn generate_vertical_patterns(start_id: usize) -> (Vec<Pattern>, usize) {
    let mut patterns = Vec::new();
    let mut id = start_id;
    for x in 0..8 {
        let positions: Vec<_> = (0..8).map(|y| Position { x, y }).collect();
        patterns.push(Pattern::from_positions(id, &positions));
        id += 1;
    }
    (patterns, id)
}

/// 左上→右下方向の4マス以上の斜めラインパターンを生成
fn generate_diagonal_down_patterns(start_id: usize) -> (Vec<Pattern>, usize) {
    let mut patterns = Vec::new();
    let mut id = start_id;

    // 上端行から開始する対角線
    for start_x in 0..8 {
        let mut positions = Vec::new();
        let (mut x, mut y) = (start_x, 0);
        while x < 8 && y < 8 {
            positions.push(Position { x, y });
            x += 1;
            y += 1;
        }
        if positions.len() >= 4 {
            patterns.push(Pattern::from_positions(id, &positions));
            id += 1;
        }
    }

    // 左端列から開始する対角線(上端は重複するので y=1以降)
    for start_y in 1..8 {
        let mut positions = Vec::new();
        let (mut x, mut y) = (0, start_y);
        while x < 8 && y < 8 {
            positions.push(Position { x, y });
            x += 1;
            y += 1;
        }
        if positions.len() >= 4 {
            patterns.push(Pattern::from_positions(id, &positions));
            id += 1;
        }
    }

    (patterns, id)
}

/// 右上→左下方向の4マス以上の斜めラインパターンを生成
fn generate_diagonal_up_patterns(start_id: usize) -> (Vec<Pattern>, usize) {
    let mut patterns = Vec::new();
    let mut id = start_id;

    // 上端行から開始する対角線
    for start_x in 0..8 {
        let mut positions = Vec::new();
        let (mut x, mut y) = (start_x, 0);
        while x < 8 && y < 8 {
            positions.push(Position { x, y });
            if x == 0 {
                break;
            }
            x -= 1;
            y += 1;
        }
        if positions.len() >= 4 {
            patterns.push(Pattern::from_positions(id, &positions));
            id += 1;
        }
    }

    // 右端列から開始する対角線(上端は重複するので y=1以降)
    for start_y in 1..8 {
        let mut positions = Vec::new();
        let (mut x, mut y) = (7, start_y);
        while x < 8 && y < 8 {
            positions.push(Position { x, y });
            if x == 0 {
                break;
            }
            x -= 1;
            y += 1;
        }
        if positions.len() >= 4 {
            patterns.push(Pattern::from_positions(id, &positions));
            id += 1;
        }
    }

    (patterns, id)
}

/// 全ライン(水平, 垂直, 4マス以上の対角)のBitPatternを生成
pub fn generate_all_line_patterns() -> Vec<Pattern> {
    let mut patterns = Vec::new();
    let mut current_id = 0;

    // 水平ライン
    {
        let (mut horiz, id) = generate_horizontal_patterns(current_id);
        patterns.append(&mut horiz);
        current_id = id;
    }

    // 垂直ライン
    {
        let (mut vert, id) = generate_vertical_patterns(current_id);
        patterns.append(&mut vert);
        current_id = id;
    }

    // 斜め(左上→右下)
    {
        let (mut diag_down, id) = generate_diagonal_down_patterns(current_id);
        patterns.append(&mut diag_down);
        current_id = id;
    }

    // 斜め(右上→左下)
    {
        let (mut diag_up, id) = generate_diagonal_up_patterns(current_id);
        patterns.append(&mut diag_up);
        // current_id = id;
    }

    patterns
}