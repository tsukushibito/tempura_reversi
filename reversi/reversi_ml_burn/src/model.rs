use burn::{
    config::Config,
    module::Module,
    nn::{loss::MseLoss, Linear, LinearConfig},
    prelude::Backend,
    tensor::{backend::AutodiffBackend, Tensor},
    train::{RegressionOutput, TrainOutput, TrainStep, ValidStep},
};

use crate::data::ReversiBatch;

#[derive(Module, Debug)]
pub struct ReversiModel<B: Backend> {
    linear: Linear<B>,
}

#[derive(Config, Debug)]
pub struct ReversiModelConfig {
    // #[config(default = 8000)]
    d_input: usize,
    #[config(default = 1)]
    d_output: usize,
}

impl ReversiModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> ReversiModel<B> {
        ReversiModel {
            linear: LinearConfig::new(self.d_input, self.d_output).init(device),
        }
    }
}

impl<B: Backend> ReversiModel<B> {
    pub fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        self.linear.forward(input)
    }

    pub fn forward_step(&self, item: ReversiBatch<B>) -> RegressionOutput<B> {
        let targets: Tensor<B, 2> = item.targets.unsqueeze_dim(1);
        let output: Tensor<B, 2> = self.forward(item.inputs);

        let loss = MseLoss::new().forward(
            output.clone(),
            targets.clone(),
            burn::nn::loss::Reduction::Mean,
        );

        RegressionOutput {
            loss,
            output,
            targets,
        }
    }
}

impl<B: AutodiffBackend> TrainStep<ReversiBatch<B>, RegressionOutput<B>> for ReversiModel<B> {
    fn step(&self, item: ReversiBatch<B>) -> burn::train::TrainOutput<RegressionOutput<B>> {
        let item = self.forward_step(item);

        TrainOutput::new(self, item.loss.backward(), item)
    }
}

impl<B: Backend> ValidStep<ReversiBatch<B>, RegressionOutput<B>> for ReversiModel<B> {
    fn step(&self, item: ReversiBatch<B>) -> RegressionOutput<B> {
        self.forward_step(item)
    }
}
