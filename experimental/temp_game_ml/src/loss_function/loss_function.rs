pub trait LossFunction {
    fn compute_loss(&self, predictions: &[f32], targets: &[f32]) -> f32;
    fn compute_gradient(&self, predictions: &[f32], targets: &[f32]) -> Vec<f32>;
}
