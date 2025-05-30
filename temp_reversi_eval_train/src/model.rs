use burn::{
    config::Config,
    module::Module,
    nn::{
        loss::{MseLoss, Reduction},
        Embedding, EmbeddingConfig,
    },
    prelude::Backend,
    tensor::{backend::AutodiffBackend, Int, Tensor},
    train::{RegressionOutput, TrainOutput, TrainStep, ValidStep},
};
use temp_reversi_eval::feature::PHASE_COUNT;

use crate::{dataset::ReversiBatch, feature_packer::FEATURE_PACKER};

#[derive(Debug, Module)]
pub struct ReversiModel<B: Backend> {
    pub feature_weights: Embedding<B>,
}

#[derive(Debug, Config)]
pub struct ReversiModelConfig {}

impl ReversiModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> ReversiModel<B> {
        let num_features = FEATURE_PACKER.packed_feature_size;
        let feature_weights =
            EmbeddingConfig::new(PHASE_COUNT as usize * num_features, 1).init(device);

        ReversiModel { feature_weights }
    }
}

impl<B: Backend> ReversiModel<B> {
    pub fn forward(&self, indices: Tensor<B, 2, Int>, values: Tensor<B, 2>) -> Tensor<B, 2> {
        // Get weights directly using combination indices
        // indices: [batch_size, sequence_length]
        // selected_weights: [batch_size, sequence_length, 1]
        let selected_weights = self.feature_weights.forward(indices);

        // Reduce dimension to [batch_size, sequence_length]
        let selected_weights = selected_weights.squeeze(2);

        // Calculate element-wise product of feature values and weights
        // values: [batch_size, sequence_length]
        // selected_weights: [batch_size, sequence_length]
        let weighted_values = values * selected_weights;

        // Compute final score through linear combination
        let score = weighted_values.sum_dim(1);

        score
    }

    pub fn forward_step(&self, item: ReversiBatch<B>) -> RegressionOutput<B> {
        let targets: Tensor<B, 2> = item.targets.clone();
        let output: Tensor<B, 2> = self.forward(item.indices, item.values);

        let loss = MseLoss::new().forward(output.clone(), targets.clone(), Reduction::Mean);

        RegressionOutput {
            output,
            loss,
            targets,
        }
    }
}

impl<B: AutodiffBackend> TrainStep<ReversiBatch<B>, RegressionOutput<B>> for ReversiModel<B> {
    fn step(&self, item: ReversiBatch<B>) -> TrainOutput<RegressionOutput<B>> {
        let item = self.forward_step(item);

        TrainOutput::new(self, item.loss.backward(), item)
    }
}

impl<B: Backend> ValidStep<ReversiBatch<B>, RegressionOutput<B>> for ReversiModel<B> {
    fn step(&self, item: ReversiBatch<B>) -> RegressionOutput<B> {
        self.forward_step(item)
    }
}
