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
        let num_phases = PHASE_COUNT as usize;
        let feature_weights = EmbeddingConfig::new(num_features, num_phases).init(device);

        ReversiModel { feature_weights }
    }
}

impl<B: Backend> ReversiModel<B> {
    pub fn forward(
        &self,
        indices: Tensor<B, 2, Int>,
        values: Tensor<B, 2>,
        phases: Tensor<B, 1, Int>,
    ) -> Tensor<B, 2> {
        // Retrieve weights for the feature vector
        // indices: [batch_size, sequence_length]
        // embedded_weights: [batch_size, sequence_length, num_phases]
        // sequence_length should match the number of patterns
        let embedded_weights = self.feature_weights.forward(indices.clone());

        // Prepare indices for gathering weights
        let batch_size = indices.dims()[0];
        let sequence_length = indices.dims()[1];
        let phases_reshaped: Tensor<B, 2, Int> = phases.reshape([batch_size, 1]);
        let phases_repeated = phases_reshaped.repeat_dim(1, sequence_length);
        let gather_indices: Tensor<B, 3, Int> = phases_repeated.unsqueeze_dim(2);
        // println!("gather_indices: {:?}", gather_indices.shape());

        // Select weights corresponding to the phase
        let selected_weights = embedded_weights.gather(2, gather_indices.clone());
        // println!("selected_weights: {:?}", selected_weights.shape());
        let selected_weights = selected_weights.squeeze(2); // [batch_size, sequence_length]

        // Compute the product of the feature vector and weights
        // values: [batch_size, sequence_length]
        // selected_weights: [batch_size, sequence_length]
        // weighted_values: [batch_size, sequence_length]
        let weighted_values = values * selected_weights;

        // Sum up to form a linear combination
        let score = weighted_values.sum_dim(1);

        score
    }

    pub fn forward_step(&self, item: ReversiBatch<B>) -> RegressionOutput<B> {
        let targets: Tensor<B, 2> = item.targets.clone();
        let output: Tensor<B, 2> = self.forward(item.indices, item.values, item.phases);

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
