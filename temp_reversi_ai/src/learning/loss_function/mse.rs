use rayon::prelude::*;

use super::LossFunction;

/// Implementation of Mean Squared Error (MSE) loss
pub struct MSELoss;

impl LossFunction for MSELoss {
    fn compute_loss(&self, predictions: &[f32], targets: &[f32]) -> Vec<f32> {
        predictions
            .par_iter()
            .zip(targets.par_iter())
            .map(|(&pred, &target)| (pred - target).powi(2))
            .collect()
    }

    fn compute_gradient(&self, predictions: &[f32], targets: &[f32]) -> Vec<f32> {
        predictions
            .par_iter()
            .zip(targets.par_iter())
            .map(|(&pred, &target)| 2.0 * (pred - target))
            .collect()
    }
}
