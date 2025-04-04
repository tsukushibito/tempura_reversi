use burn::{
    data::{dataloader::batcher::Batcher, dataset::Dataset},
    prelude::*,
};
use temp_reversi_eval::feature::Feature;

#[derive(Debug, Clone, Default)]
pub struct ReversiSample {
    pub feature: Feature,
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
        let inputs: Vec<Tensor<B, 2>> = samples
            .iter()
            .map(|s| feature_to_tensor(&s.feature, &self.device))
            .collect();
        let inputs = Tensor::cat(inputs, 0);

        let targets: Vec<Tensor<B, 1>> = samples
            .iter()
            .map(|s| stone_diff_to_tensor(s.stone_diff, &self.device))
            .collect();
        let targets = Tensor::cat(targets, 0);
        let targets = targets.unsqueeze_dim(1);

        let phases = samples.iter().map(|s| s.feature.phase).collect::<Vec<_>>();
        ReversiBatch {
            inputs,
            targets,
            phases,
        }
    }
}

fn feature_to_tensor<B: Backend>(feature: &Feature, device: &Device<B>) -> Tensor<B, 2> {
    let indices: Vec<f32> = feature.indices.iter().map(|&i| i as f32).collect();
    Tensor::<B, 1>::from_floats(indices.as_slice(), device).unsqueeze()
}

fn stone_diff_to_tensor<B: Backend>(stone_diff: i8, device: &Device<B>) -> Tensor<B, 1> {
    Tensor::<B, 1>::from_floats([stone_diff as f32], device)
}
