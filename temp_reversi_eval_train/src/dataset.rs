use burn::{
    data::{dataloader::batcher::Batcher, dataset::Dataset},
    prelude::*,
};
use rayon::prelude::*;
use serde::de::value;
use temp_reversi_eval::feature::Feature;

use crate::feature_packer::FEATURE_PACKER;

#[derive(Debug, Clone, Default)]
pub struct ReversiSample {
    pub packed_feature: Feature,
    pub stone_diff: i8,
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
    pub indices: Tensor<B, 2, Int>,
    pub values: Tensor<B, 2>,
    pub phases: Tensor<B, 1, Int>,
    pub targets: Tensor<B, 2>,
}

impl<B: Backend> ReversiBatcher<B> {
    pub fn new(device: Device<B>) -> Self {
        Self { device }
    }
}

impl<B: Backend> Batcher<ReversiSample, ReversiBatch<B>> for ReversiBatcher<B> {
    fn batch(&self, samples: Vec<ReversiSample>) -> ReversiBatch<B> {
        let mut indices = Vec::new();
        let mut values = Vec::new();
        let mut phases = Vec::new();
        let mut targets = Vec::new();
        for s in samples {
            let (idxs, vals) = FEATURE_PACKER.packed_feature_to_sparse_vector(&s.packed_feature);
            let index_tensor: Tensor<B, 1, Int> = Tensor::from_ints(idxs.as_slice(), &self.device);
            let index_tensor: Tensor<B, 2, Int> = index_tensor.unsqueeze();
            indices.push(index_tensor);

            let value_tensor: Tensor<B, 1> = Tensor::from_floats(vals.as_slice(), &self.device);
            let value_tensor: Tensor<B, 2> = value_tensor.unsqueeze();
            values.push(value_tensor);

            let phase_tensor: Tensor<B, 1, Int> =
                Tensor::from_ints([s.packed_feature.phase as i32], &self.device);
            phases.push(phase_tensor);

            let target_tensor: Tensor<B, 1> =
                Tensor::from_floats([s.stone_diff as f32], &self.device);
            let target_tensor: Tensor<B, 2> = target_tensor.unsqueeze();
            targets.push(target_tensor);
        }

        let indices = Tensor::cat(indices, 0);
        let values = Tensor::cat(values, 0);
        let phases = Tensor::cat(phases, 0);
        let targets = Tensor::cat(targets, 0);

        ReversiBatch {
            indices,
            values,
            phases,
            targets,
        }
    }
}
