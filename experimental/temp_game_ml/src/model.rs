use crate::tensor::Tensor;

pub trait Model {
    fn forward(&self, inputs: &[&[f32]]) -> Self::Output;
    fn parameters(&self) -> (&[f32], &[f32]);
    fn parameterss_mut(&mut self) -> (&mut [f32], &mut [f32]);
    fn backword(
        &mut self,
        input: &Self::Input,
        grad_output: &Self::Output,
    ) -> (Self::WeightGrads, Self::BiasGrads);
}
