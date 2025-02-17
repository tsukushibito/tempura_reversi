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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_loss() {
        let mse = MSELoss;
        let predictions = [2.0, 3.0];
        let targets = [1.0, 3.0];
        let loss = mse.compute_loss(&predictions, &targets);
        assert_eq!(loss, vec![1.0, 0.0]);
    }

    #[test]
    fn test_compute_gradient() {
        let mse = MSELoss;
        let predictions = [2.0, 3.0];
        let targets = [1.0, 3.0];
        let grad = mse.compute_gradient(&predictions, &targets);
        assert_eq!(grad, vec![2.0, 0.0]);
    }
}
