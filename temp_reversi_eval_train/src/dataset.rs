use burn::{
    data::{dataloader::batcher::Batcher, dataset::Dataset},
    prelude::*,
};
use rayon::prelude::*;
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
    pub inputs: Tensor<B, 2>,
    pub targets: Tensor<B, 2>,
    pub phases: Vec<u8>,
}

impl<B: Backend> ReversiBatcher<B> {
    pub fn new(device: Device<B>) -> Self {
        Self { device }
    }
}

impl<B: Backend> Batcher<ReversiSample, ReversiBatch<B>> for ReversiBatcher<B> {
    fn batch(&self, samples: Vec<ReversiSample>) -> ReversiBatch<B> {
        let (input_tensors, target_tensors_and_phases): (Vec<_>, Vec<(_, _)>) = samples
            .par_iter()
            .map(|s| {
                (
                    feature_to_tensor(&s.packed_feature, &self.device),
                    (
                        stone_diff_to_tensor(s.stone_diff, &self.device),
                        s.packed_feature.phase,
                    ),
                )
            })
            .unzip();
        let (target_tensors, phases): (Vec<_>, Vec<_>) =
            target_tensors_and_phases.into_iter().unzip();

        let inputs = Tensor::cat(input_tensors, 0);
        let targets = Tensor::cat(target_tensors, 0).unsqueeze_dim(1);

        ReversiBatch {
            inputs,
            targets,
            phases,
        }
    }
}

fn feature_to_tensor<B: Backend>(feature: &Feature, device: &Device<B>) -> Tensor<B, 2> {
    let feature_vector = FEATURE_PACKER.packed_feature_to_vector(feature);
    let feature_vector: Vec<f32> = feature_vector.iter().map(|&x| x as f32).collect();
    Tensor::<B, 1>::from_floats(feature_vector.as_slice(), device).unsqueeze()
}

fn stone_diff_to_tensor<B: Backend>(stone_diff: i8, device: &Device<B>) -> Tensor<B, 1> {
    Tensor::<B, 1>::from_floats([stone_diff as f32], device)
}
