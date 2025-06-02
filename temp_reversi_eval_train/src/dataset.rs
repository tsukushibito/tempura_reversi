use burn::{
    data::{dataloader::batcher::Batcher, dataset::Dataset},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use temp_reversi_core::Bitboard;
use temp_reversi_eval::feature::extract_feature;

use crate::feature_packer::FEATURE_PACKER;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReversiSample {
    pub black_bits: u64,
    pub white_bits: u64,
    pub stone_diff: i8,
}

impl ReversiSample {
    pub fn feature_vector(&self) -> (Vec<i32>, Vec<f32>) {
        let board = Bitboard::new(self.black_bits, self.white_bits);
        let feature = extract_feature(&board);
        let packed_feature = FEATURE_PACKER.pack(&feature);

        let mut indices = Vec::new();
        let mut values = Vec::new();
        for (i, &index) in packed_feature.indices.iter().enumerate() {
            let base_pattern_index = i / 4;
            let index_offset = FEATURE_PACKER.index_offsets[base_pattern_index] as usize;
            let absolute_index = index_offset + index as usize;
            if absolute_index >= FEATURE_PACKER.packed_feature_size {
                panic!("Packed feature index out of bounds.");
            }
            indices.push(absolute_index as i32);
            values.push(1.0);
        }

        (indices, values)
    }

    pub fn phase(&self) -> i32 {
        let board = Bitboard::new(self.black_bits, self.white_bits);
        let (black, white) = board.count_stones();
        (black + white) as i32
    }
}

pub struct ReversiDataset {
    samples: Vec<ReversiSample>,
}

impl ReversiDataset {
    pub fn new(samples: Vec<ReversiSample>) -> Self {
        Self { samples }
    }
}

impl Dataset<ReversiSample> for ReversiDataset {
    fn len(&self) -> usize {
        self.samples.len()
    }

    fn get(&self, index: usize) -> Option<ReversiSample> {
        self.samples.get(index).cloned()
    }
}

#[derive(Debug, Clone)]
pub struct ReversiBatcher<B: Backend> {
    device: Device<B>,
}

#[derive(Debug, Clone)]
pub struct ReversiBatch<B: Backend> {
    /// Indices of the features in the packed feature vector.
    /// The indices are structured as follows:
    /// Phase 0: [feature0, feature1, feature2, ..., featureN]
    /// Phase 1: [feature0, feature1, feature2, ..., featureN]
    /// Phase 2: [feature0, feature1, feature2, ..., featureN]
    /// where features are arranged for each phase,
    /// and the index represents the position when flattened into a one-dimensional array.
    /// Index = phase * FEATURE_PACKER.packed_feature_size + feature_index
    pub indices: Tensor<B, 2, Int>,
    pub values: Tensor<B, 2>,
    pub targets: Tensor<B, 2>,
}

impl<B: Backend> ReversiBatcher<B> {
    pub fn new(device: Device<B>) -> Self {
        Self { device }
    }
}

impl<B: Backend> Batcher<B, ReversiSample, ReversiBatch<B>> for ReversiBatcher<B> {
    fn batch(&self, samples: Vec<ReversiSample>, _device: &B::Device) -> ReversiBatch<B> {
        let mut indices = Vec::new();
        let mut values = Vec::new();
        let mut targets = Vec::new();
        for s in samples {
            let (idxs, vals) = s.feature_vector();
            let phase = s.phase();
            let combined_idxs: Vec<i32> = idxs
                .iter()
                .map(|&i| phase * FEATURE_PACKER.packed_feature_size as i32 + i)
                .collect();
            let index_tensor: Tensor<B, 1, Int> =
                Tensor::from_ints(combined_idxs.as_slice(), &self.device);
            let index_tensor: Tensor<B, 2, Int> = index_tensor.unsqueeze();
            indices.push(index_tensor);

            let value_tensor: Tensor<B, 1> = Tensor::from_floats(vals.as_slice(), &self.device);
            let value_tensor: Tensor<B, 2> = value_tensor.unsqueeze();
            values.push(value_tensor);

            let target_tensor: Tensor<B, 1> =
                Tensor::from_floats([s.stone_diff as f32], &self.device);
            let target_tensor: Tensor<B, 2> = target_tensor.unsqueeze();
            targets.push(target_tensor);
        }

        let indices = Tensor::cat(indices, 0);
        let values = Tensor::cat(values, 0);
        let targets = Tensor::cat(targets, 0);

        ReversiBatch {
            indices,
            values,
            targets,
        }
    }
}
