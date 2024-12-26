use super::{Loss, LossFunction};

pub struct MSE;

impl MSE {
    pub fn new() -> Self {
        MSE
    }
}

impl LossFunction for MSE {
    fn compute(&self, outputs: &[f32], targets: &[f32]) -> Loss {
        assert_eq!(
            outputs.len(),
            targets.len(),
            "Outputs and targets must have the same length."
        );

        let mut loss_value = 0.0;
        let mut grad = Vec::with_capacity(outputs.len());

        for (&output, &target) in outputs.iter().zip(targets.iter()) {
            let error = output - target;
            loss_value += error * error;
            grad.push(2.0 * error);
        }

        loss_value /= outputs.len() as f32; // 平均を取る
        for g in grad.iter_mut() {
            *g /= outputs.len() as f32;
        }

        Loss {
            value: loss_value,
            grad,
        }
    }
}
