use super::{Loss, LossFunction};

#[derive(Debug, Default, Clone)]
pub struct Mse;

impl Mse {
    pub fn new() -> Self {
        Mse
    }
}

impl LossFunction for Mse {
    fn compute(&self, pred: &[f32], targets: &[f32]) -> Loss {
        assert_eq!(
            pred.len(),
            targets.len(),
            "Outputs and targets must have the same length."
        );

        let mut loss_value = 0.0;
        let mut grad = Vec::with_capacity(pred.len());

        for (&output, &target) in pred.iter().zip(targets.iter()) {
            let error = output - target;
            loss_value += error * error;
            grad.push(2.0 * error);
        }

        loss_value /= pred.len() as f32; // 平均を取る
        for g in grad.iter_mut() {
            *g /= pred.len() as f32;
        }

        Loss {
            value: loss_value,
            grad,
        }
    }
}
