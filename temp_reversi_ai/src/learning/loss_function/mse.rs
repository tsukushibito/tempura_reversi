use super::LossFunction;

/// Implementation of Mean Squared Error (MSE) loss
pub struct MSELoss;

impl LossFunction for MSELoss {
    fn compute_loss(&self, prediction: f32, target: f32) -> f32 {
        (prediction - target).powi(2)
    }

    fn compute_gradient(&self, prediction: f32, target: f32) -> f32 {
        2.0 * (prediction - target)
    }
}
