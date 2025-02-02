/// Trait for loss functions used in model training
pub trait LossFunction {
    /// Calculates the loss for a batch of predictions and target values
    fn compute_loss(&self, predictions: &[f32], targets: &[f32]) -> Vec<f32>;

    /// Calculates the gradients of the loss with respect to the predictions
    fn compute_gradient(&self, predictions: &[f32], targets: &[f32]) -> Vec<f32>;
}

mod mse;
pub use mse::*;
