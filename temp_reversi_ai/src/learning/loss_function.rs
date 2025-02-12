/// Trait for loss functions used in model training
pub trait LossFunction {
    /// Calculates the loss for a batch of predictions and target values
    fn compute_loss(&self, predictions: &[f32], targets: &[f32]) -> Vec<f32>;

    /// Calculates the gradients of the loss with respect to the predictions
    fn compute_gradient(&self, predictions: &[f32], targets: &[f32]) -> Vec<f32>;

    /// Default implementation for computing loss by phase.
    ///
    /// Returns a tuple:
    /// - The full vector of losses for each sample.
    /// - A Vec of length 60 where each element is a Vec<f32> containing all loss values for that phase.
    fn compute_loss_by_phase(
        &self,
        predictions: &[f32],
        targets: &[f32],
        phases: &[usize],
    ) -> (Vec<f32>, Vec<Vec<f32>>) {
        assert_eq!(
            predictions.len(),
            targets.len(),
            "Predictions and targets must have the same length"
        );
        assert_eq!(
            predictions.len(),
            phases.len(),
            "Predictions and phases must have the same length"
        );

        let losses = self.compute_loss(predictions, targets);
        let mut phase_losses: Vec<Vec<f32>> = vec![Vec::new(); 60];

        for (loss, &phase) in losses.iter().zip(phases.iter()) {
            if phase < 60 {
                phase_losses[phase].push(*loss);
            }
        }

        (losses, phase_losses)
    }
}

mod mse;

pub use mse::*;
