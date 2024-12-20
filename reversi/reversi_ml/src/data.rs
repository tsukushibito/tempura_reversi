use burn::{data::dataloader::batcher::Batcher, prelude::Backend, tensor::Tensor};

pub struct ReversiItem {
    pub feature: Vec<f32>,
    pub value: f32,
}

#[derive(Clone)]
pub struct ReversiBatcher<B: Backend> {
    device: B::Device,
}

impl<B: Backend> ReversiBatcher<B> {
    pub fn new(device: B::Device) -> Self {
        Self { device }
    }
}

#[derive(Clone, Debug)]
pub struct ReversiBatch<B: Backend> {
    pub features: Tensor<B, 2>,
    pub values: Tensor<B, 2>,
}

impl<B: Backend> Batcher<ReversiItem, ReversiBatch<B>> for ReversiBatcher<B> {
    fn batch(&self, items: Vec<ReversiItem>) -> ReversiBatch<B> {
        let features = items
            .iter()
            .map(|item| Tensor::<B, 2>::from_floats(item.feature.as_slice(), &self.device))
            .collect::<Vec<_>>();

        let values = items
            .iter()
            .map(|item| Tensor::<B, 2>::from_floats([item.value], &self.device))
            .collect::<Vec<_>>();

        let features = Tensor::cat(features, 0).to_device(&self.device);

        let values = Tensor::cat(values, 0).to_device(&self.device);

        ReversiBatch { features, values }
    }
}
