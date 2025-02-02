/// Trait for loss functions used in model training
pub trait LossFunction {
    /// Calculates the loss given the prediction and target value
    fn compute_loss(&self, prediction: f32, target: f32) -> f32;

    /// Calculates the gradient of the loss with respect to the prediction
    fn compute_gradient(&self, prediction: f32, target: f32) -> f32;
}

mod mse;
pub use mse::*;
