use super::LossFunction;

pub struct MSELoss;

impl LossFunction for MSELoss {
    fn compute_loss(&self, predictions: &[f32], targets: &[f32]) -> f32 {
        predictions
            .iter()
            .zip(targets.iter())
            .map(|(pred, target)| (pred - target).powi(2))
            .sum::<f32>()
            / predictions.len() as f32
    }

    fn compute_gradient(&self, predictions: &[f32], targets: &[f32]) -> Vec<f32> {
        predictions
            .iter()
            .zip(targets.iter())
            .map(|(pred, target)| 2.0 * (pred - target) / predictions.len() as f32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let predictions = vec![0.2, 0.5, 0.8];
        let targets = vec![0.0, 0.6, 1.0];

        let mse_loss = MSELoss;

        let loss = mse_loss.compute_loss(&predictions, &targets);
        let gradient = mse_loss.compute_gradient(&predictions, &targets);

        println!("loss: {}", loss);
        println!("gradient: {:?}", gradient);
    }
}
