use std::{collections::HashMap, fs::File, io::Read};

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{bit_board::BitBoard, Position, SparseVector};

pub const PATTERN_ROTATION_0: usize = 0;
pub const PATTERN_ROTATION_90: usize = 1;
pub const PATTERN_ROTATION_180: usize = 2;
pub const PATTERN_ROTATION_270: usize = 3;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Pattern {
    pub id: usize,
    pub masks: [u64; 4],
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

        Self { id, masks }
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
            // println!("black={:b}", black_pattern);
            // println!("white={:b}", white_pattern);

            let mut idx = 0;
            let mut mask_copy = *mask;

            let mut i = 0;
            while mask_copy != 0 {
                // println!("i={}, mask_copy={:b}", i, mask_copy);

                // 最下位のセットビットの抽出
                let bit = mask_copy & (!mask_copy + 1);

                let val = if (black_pattern & bit) != 0 {
                    1
                } else if (white_pattern & bit) != 0 {
                    2
                } else {
                    0
                };

                idx += 3usize.pow(i) * val;
                i += 1;

                // 最下位のセットビットを除去
                mask_copy &= mask_copy - 1;
            }

            *index = idx;
        });
        indices
    }

    pub fn feature(&self, board: &BitBoard) -> SparseVector {
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

        SparseVector::new(indices, values, self.state_count()).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_positions() {
        let positions = vec![
            Position { x: 0, y: 0 },
            Position { x: 1, y: 0 },
            Position { x: 0, y: 1 },
        ];
        let pattern = Pattern::from_positions(1, &positions);

        let expected_masks = [
            0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001_0000_0011, // 回転なし
            0b0000_0011_0000_0001_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000, // 90度回転
            0b1100_0000_1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000, // 180度回転
            0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000_0000_1100_0000, // 270度回転
        ];
        (0..4).for_each(|i| {
            assert_eq!(
                pattern.masks[i],
                expected_masks[i],
                "Mask at rotation {} incorrect",
                i * 90
            );
        });
    }

    #[test]
    fn test_state_count() {
        let positions = vec![
            Position { x: 0, y: 0 },
            Position { x: 1, y: 0 },
            Position { x: 0, y: 1 },
        ];
        let pattern = Pattern::from_positions(1, &positions);
        assert_eq!(pattern.state_count(), 3 * 3 * 3);
    }

    #[test]
    fn test_state_indices() {
        let positions = vec![
            Position { x: 0, y: 0 },
            Position { x: 1, y: 0 },
            Position { x: 0, y: 1 },
        ];
        let pattern = Pattern::from_positions(1, &positions);

        let board = BitBoard {
            black: 0b0000_0001_0000_0010,
            white: 0b0011_0000_0000_0001,
        };

        let indices = pattern.state_indices(&board);
        assert_eq!(
            indices[0],
            2 * 3usize.pow(0) + 3usize.pow(1) + 3usize.pow(2)
        );
        assert_eq!(indices[1], 0);
        assert_eq!(indices[2], 0);
        assert_eq!(indices[3], 0);
    }

    #[test]
    fn test_feature() {
        let positions = vec![
            Position { x: 0, y: 0 },
            Position { x: 1, y: 0 },
            Position { x: 0, y: 1 },
        ];
        let pattern = Pattern::from_positions(1, &positions);

        let board = BitBoard {
            black: 0b0000_0001_0000_0010,
            white: 0b0011_0000_0000_0001,
        };

        let feature = pattern.feature(&board);
        println!("feature.indices[0]={}", feature.indices()[0]);
        println!("feature.indices[1]={}", feature.indices()[1]);

        assert_eq!(feature.indices().len(), 2);
        assert_eq!(feature.values().len(), 2);

        assert_eq!(feature.indices()[0], 0);
        assert_eq!(feature.values()[0], 3.0);

        assert_eq!(feature.indices()[1], 14);
        assert_eq!(feature.values()[1], 1.0);
    }
}
