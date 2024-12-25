use std::{collections::HashMap, fs::File, io::Read};

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{bit_board::BitBoard, Position};

use super::sparse_feature::SparseFeature;

pub const PATTERN_ROTATION_0: usize = 0;
pub const PATTERN_ROTATION_90: usize = 1;
pub const PATTERN_ROTATION_180: usize = 2;
pub const PATTERN_ROTATION_270: usize = 3;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Pattern {
    pub id: usize,
    pub masks: [u64; 4],
    pub values: Vec<f32>,
}

impl Pattern {
    pub fn from_positions(id: usize, positions: &[Position]) -> Self {
        let mut masks = [0u64; 4];
        let mut positions = positions.to_vec();

        masks.iter_mut().for_each(|mask| {
            for pos in &positions {
                let bit_index = pos.to_index();
                *mask |= 1 << bit_index;
            }
            positions.iter_mut().for_each(|p| p.rotate_90());
        });

        let values = vec![0.0; 3usize.pow(masks[0].count_ones())];

        Self { id, masks, values }
    }

    pub fn state_count(&self) -> usize {
        3usize.pow(self.masks[0].count_ones())
    }

    pub fn state_indices(&self, board: &BitBoard) -> [usize; 4] {
        let mut indices = [0usize; 4];
        indices.iter_mut().enumerate().for_each(|(i, index)| {
            let mask = &self.masks[i];
            let black_pattern = board.black & mask;
            let white_pattern = board.white & mask;

            let mut idx = 0;
            let mut mask_copy = *mask;

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

            *index = idx;
        });
        indices
    }

    pub fn feature(&self, board: &BitBoard) -> SparseFeature {
        let mut index_count: HashMap<usize, f32> = HashMap::new();

        for index in self.state_indices(board) {
            *index_count.entry(index).or_insert(0.0) += 1.0;
        }

        let mut indices = Vec::new();
        let mut values = Vec::new();

        for (index, value) in index_count {
            indices.push(index);
            values.push(value);
        }

        SparseFeature::new(indices, values, self.state_count()).unwrap_or_default()
    }

    pub fn value(&self, board: &BitBoard) -> f32 {
        let mut value = 0.0;

        for i in self.state_indices(board) {
            value += self.values[i];
        }

        value
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
        let patterns = generate_patterns();
        let mut rng = rand::thread_rng();
        let mut index_offsets = Vec::new();
        let mut index_offset = 0;
        let mut scores = Vec::new();

        patterns.iter().enumerate().for_each(|(id, p)| {
            assert!(id == p.id, "idは連番");

            let length = p.state_count();

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

    pub fn load(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        let table: Self = bincode::deserialize(&buf)?;

        Ok(table)
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
        let state_index = pattern.state_indices(board);
        let index_offset = self.index_offsets[pattern.id];
        index_offset + state_index[0]
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

pub fn generate_edge_x_patterns(start_id: usize) -> (Vec<Pattern>, usize) {
    let mut patterns = Vec::new();
    let mut id = start_id;

    // エッジ座標定義
    let top_edge_positions: Vec<_> = (0..8).map(|x| Position { x, y: 0 }).collect();
    let bottom_edge_positions: Vec<_> = (0..8).map(|x| Position { x, y: 7 }).collect();
    let left_edge_positions: Vec<_> = (0..8).map(|y| Position { x: 0, y }).collect();
    let right_edge_positions: Vec<_> = (0..8).map(|y| Position { x: 7, y }).collect();

    // Xマス定義(隅の内側)
    let top_x_positions = vec![Position { x: 1, y: 1 }, Position { x: 6, y: 1 }];
    let bottom_x_positions = vec![Position { x: 1, y: 6 }, Position { x: 6, y: 6 }];
    let left_x_positions = vec![Position { x: 1, y: 1 }, Position { x: 1, y: 6 }];
    let right_x_positions = vec![Position { x: 6, y: 1 }, Position { x: 6, y: 6 }];

    // エッジ + Xマスパターン
    {
        let mut top_with_x = top_edge_positions.clone();
        top_with_x.extend(&top_x_positions);
        patterns.push(Pattern::from_positions(id, &top_with_x));
        id += 1;

        let mut bottom_with_x = bottom_edge_positions.clone();
        bottom_with_x.extend(&bottom_x_positions);
        patterns.push(Pattern::from_positions(id, &bottom_with_x));
        id += 1;

        let mut left_with_x = left_edge_positions.clone();
        left_with_x.extend(&left_x_positions);
        patterns.push(Pattern::from_positions(id, &left_with_x));
        id += 1;

        let mut right_with_x = right_edge_positions.clone();
        right_with_x.extend(&right_x_positions);
        patterns.push(Pattern::from_positions(id, &right_with_x));
        id += 1;
    }

    (patterns, id)
}

pub fn generate_corner_3x3_patterns(start_id: usize) -> (Vec<Pattern>, usize) {
    let mut patterns = Vec::new();
    let mut id = start_id;

    // 左上コーナー
    let top_left_positions: Vec<Position> = (0..3)
        .flat_map(|x| (0..3).map(move |y| Position { x, y }))
        .collect();
    patterns.push(Pattern::from_positions(id, &top_left_positions));
    id += 1;

    // 右上コーナー
    let top_right_positions: Vec<Position> = (5..8)
        .flat_map(|x| (0..3).map(move |y| Position { x, y }))
        .collect();
    patterns.push(Pattern::from_positions(id, &top_right_positions));
    id += 1;

    // 左下コーナー
    let bottom_left_positions: Vec<Position> = (0..3)
        .flat_map(|x| (5..8).map(move |y| Position { x, y }))
        .collect();
    patterns.push(Pattern::from_positions(id, &bottom_left_positions));
    id += 1;

    // 右下コーナー
    let bottom_right_positions: Vec<Position> = (5..8)
        .flat_map(|x| (5..8).map(move |y| Position { x, y }))
        .collect();
    patterns.push(Pattern::from_positions(id, &bottom_right_positions));
    id += 1;

    (patterns, id)
}

pub fn generate_corner_2x4_patterns(start_id: usize) -> (Vec<Pattern>, usize) {
    let mut patterns = Vec::new();
    let mut id = start_id;

    // 左上コーナー (0,0)
    // 水平(2x4): x=0..3, y=0..1
    {
        let positions: Vec<Position> = (0..4)
            .flat_map(|x| (0..2).map(move |y| Position { x, y }))
            .collect();
        patterns.push(Pattern::from_positions(id, &positions));
        id += 1;
    }
    // 垂直(4x2): x=0..1, y=0..3
    {
        let positions: Vec<Position> = (0..2)
            .flat_map(|x| (0..4).map(move |y| Position { x, y }))
            .collect();
        patterns.push(Pattern::from_positions(id, &positions));
        id += 1;
    }

    // 右上コーナー (7,0)
    // 水平(2x4): x=4..7, y=0..1
    {
        let positions: Vec<Position> = (4..8)
            .flat_map(|x| (0..2).map(move |y| Position { x, y }))
            .collect();
        patterns.push(Pattern::from_positions(id, &positions));
        id += 1;
    }
    // 垂直(4x2): x=6..7, y=0..3
    {
        let positions: Vec<Position> = (6..8)
            .flat_map(|x| (0..4).map(move |y| Position { x, y }))
            .collect();
        patterns.push(Pattern::from_positions(id, &positions));
        id += 1;
    }

    // 左下コーナー (0,7)
    // 水平(2x4): x=0..3, y=6..7
    {
        let positions: Vec<Position> = (0..4)
            .flat_map(|x| (6..8).map(move |y| Position { x, y }))
            .collect();
        patterns.push(Pattern::from_positions(id, &positions));
        id += 1;
    }
    // 垂直(4x2): x=0..1, y=4..7
    {
        let positions: Vec<Position> = (0..2)
            .flat_map(|x| (4..8).map(move |y| Position { x, y }))
            .collect();
        patterns.push(Pattern::from_positions(id, &positions));
        id += 1;
    }

    // 右下コーナー (7,7)
    // 水平(2x4): x=4..7, y=6..7
    {
        let positions: Vec<Position> = (4..8)
            .flat_map(|x| (6..8).map(move |y| Position { x, y }))
            .collect();
        patterns.push(Pattern::from_positions(id, &positions));
        id += 1;
    }
    // 垂直(4x2): x=6..7, y=4..7
    {
        let positions: Vec<Position> = (6..8)
            .flat_map(|x| (4..8).map(move |y| Position { x, y }))
            .collect();
        patterns.push(Pattern::from_positions(id, &positions));
        id += 1;
    }

    (patterns, id)
}

pub fn generate_patterns() -> Vec<Pattern> {
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
        current_id = id;
    }

    // エッジ+Xマス
    {
        let (mut edge_x, id) = generate_edge_x_patterns(current_id);
        patterns.append(&mut edge_x);
        current_id = id;
    }

    // コーナー3x3
    {
        let (mut corner, id) = generate_corner_3x3_patterns(current_id);
        patterns.append(&mut corner);
        current_id = id;
    }

    // コーナー2x4
    {
        let (mut corner, id) = generate_corner_2x4_patterns(current_id);
        patterns.append(&mut corner);
        current_id = id;
    }

    patterns
}
