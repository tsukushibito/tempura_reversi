use std::collections::HashMap;

use burn::{
    config::Config,
    module::Module,
    nn::{
        loss::{MseLoss, Reduction},
        Linear, LinearConfig,
    },
    prelude::Backend,
    tensor::{backend::AutodiffBackend, Int, Tensor},
    train::{RegressionOutput, TrainOutput, TrainStep, ValidStep},
};

use crate::{dataset::ReversiBatch, feature_packer::FEATURE_PACKER};

#[derive(Debug, Module)]
pub struct ReversiModel<B: Backend> {
    pub linears: Vec<Linear<B>>,
}

#[derive(Debug, Config)]
pub struct ReversiModelConfig {}

impl ReversiModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> ReversiModel<B> {
        let mut linears = Vec::new();
        for i in 0..64 {
            let linear = LinearConfig::new(FEATURE_PACKER.packed_feature_size, 1).init(device);
            linears.push(linear);
        }
        ReversiModel { linears }
    }
}

impl<B: Backend> ReversiModel<B> {
    pub fn forward_single_phase(&self, input: Tensor<B, 2>, phases: u8) -> Tensor<B, 2> {
        self.linears[phases as usize].forward(input)
    }

    pub fn forward_step(&self, item: ReversiBatch<B>) -> RegressionOutput<B> {
        let device = item.inputs.device();

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

        let loss =
            MseLoss::new().forward(all_outputs.clone(), final_targets.clone(), Reduction::Mean);

        RegressionOutput {
            loss,
            output: all_outputs,
            targets: final_targets,
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
