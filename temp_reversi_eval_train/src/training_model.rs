use burn::{
    config::Config,
    module::Module,
    nn::{Linear, LinearConfig},
    prelude::Backend,
    tensor::Tensor,
    train::RegressionOutput,
};

use crate::dataset::ReversiBatch;

#[derive(Debug, Module)]
pub struct TrainingModel<B: Backend> {
    pub linears: Vec<Linear<B>>,
}

#[derive(Debug, Config)]
pub struct TrainingModelConfig {
    pub feature_size: usize,
}

impl TrainingModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> TrainingModel<B> {
        let mut linears = Vec::new();
        for i in 0..64 {
            let linear = LinearConfig::new(self.feature_size, 1).init(device);
            linears.push(linear);
        }
        TrainingModel { linears }
    }
}

impl<B: Backend> TrainingModel<B> {
    pub fn forward(&self, input: Tensor<B, 2>, phases: Vec<u8>) -> Tensor<B, 2> {}
}
