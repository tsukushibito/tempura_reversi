use std::collections::HashMap;

use burn::{
    config::Config,
    module::Module,
    nn::{Linear, LinearConfig},
    prelude::Backend,
    tensor::{backend::AutodiffBackend, Int, Tensor},
    train::{RegressionOutput, TrainOutput, TrainStep},
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
    pub fn forward_single_phase(&self, input: Tensor<B, 2>, phases: u8) -> Tensor<B, 2> {
        self.linears[phases as usize].forward(input)
    }
}

impl<B: AutodiffBackend> TrainStep<ReversiBatch<B>, Tensor<B, 1>> for TrainingModel<B> {
    fn step(&self, item: ReversiBatch<B>) -> TrainOutput<Tensor<B, 1>> {
        let device = item.inputs.device();
        let batch_size = item.inputs.dims()[0];

        let mut phase_indices: HashMap<u8, Vec<usize>> = HashMap::new();
        for (i, phase) in item.phases.iter().enumerate() {
            phase_indices.entry(*phase).or_insert_with(Vec::new).push(i);
        }

        let mut all_outputs = Tensor::zeros_like(&item.targets);
        let mut final_targets = Tensor::zeros_like(&item.targets);

        for (phase, indices) in phase_indices {
            let indices_vec: Vec<i32> = indices.iter().map(|&i| i as i32).collect();
            let indices_tensor = Tensor::<B, 1, Int>::from_ints(indices_vec.as_slice(), &device);

            let phase_inputs = item.inputs.clone().select(0, indices_tensor.clone());
            let phase_targets = item.targets.clone().select(0, indices_tensor.clone());

            let phase_outputs = self.forward_single_phase(phase_inputs, phase);

            all_outputs = all_outputs.select_assign(0, indices_tensor.clone(), phase_outputs);
            final_targets = final_targets.select_assign(0, indices_tensor.clone(), phase_targets);
        }

        todo!()
    }
}
