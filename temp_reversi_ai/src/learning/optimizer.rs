/// Trait for optimizers used in model training
pub trait Optimizer {
    /// Updates the model weights and bias based on gradients
    fn update(&mut self, weights: &mut [f32], bias: &mut f32, gradients: &[f32], bias_grad: f32);
}

mod adam;
pub use adam::*;
