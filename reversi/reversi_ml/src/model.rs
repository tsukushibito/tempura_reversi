use burn::{
    config::Config,
    module::Module,
    nn::{Linear, LinearConfig},
    prelude::Backend,
    tensor::Tensor,
};

#[derive(Module, Debug)]
pub struct ReversiModel<B: Backend> {
    linear: Linear<B>,
}

#[derive(Config, Debug)]
pub struct ReversiModelConfig {
    d_input: usize,
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
}
