use std::f32::EPSILON;

use super::{Loss, LossFunction};

pub struct CrossEntropy;

impl CrossEntropy {
    pub fn new() -> Self {
        CrossEntropy
    }
}

impl LossFunction for CrossEntropy {
    fn compute(&self, outputs: &[f32], targets: &[f32]) -> Loss {
        assert_eq!(
            outputs.len(),
            targets.len(),
            "Outputs and targets must have the same length."
        );

        // ソフトマックスの計算
        let max_output = outputs.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let mut exp_outputs = Vec::with_capacity(outputs.len());
        let mut sum_exp = 0.0;

        for &output in outputs.iter() {
            let exp_val = (output - max_output).exp();
            exp_outputs.push(exp_val);
            sum_exp += exp_val;
        }

        // ソフトマックス出力
        let softmax: Vec<f32> = exp_outputs.iter().map(|&x| x / sum_exp).collect();

        // クロスエントロピー損失の計算
        let mut loss_value = 0.0;
        let mut grad = Vec::with_capacity(outputs.len());

        for (&s, &t) in softmax.iter().zip(targets.iter()) {
            loss_value -= t * (s + EPSILON).ln();
            grad.push(s - t);
        }

        Loss {
            value: loss_value,
            grad,
        }
    }
}