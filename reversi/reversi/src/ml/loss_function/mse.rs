use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use super::{Loss, LossFunction};

#[derive(Debug, Default, Clone)]
pub struct Mse;

impl Mse {
    pub fn new() -> Self {
        Mse
    }
}

impl LossFunction for Mse {
    // fn compute(&self, preds: &[f32], targets: &[f32]) -> Loss {
    //     assert_eq!(
    //         preds.len(),
    //         targets.len(),
    //         "Outputs and targets must have the same length."
    //     );

    //     let mut loss_value = 0.0;
    //     let mut grad = Vec::with_capacity(preds.len());

    //     for (&pred, &target) in preds.iter().zip(targets.iter()) {
    //         let error = pred - target;
    //         // println!("error = {}", error);
    //         loss_value += error * error;
    //         grad.push(2.0 * error);
    //     }

    //     loss_value /= preds.len() as f32; // 平均を取る
    //     for g in grad.iter_mut() {
    //         *g /= preds.len() as f32;
    //     }

    //     Loss {
    //         value: loss_value,
    //         grad,
    //     }
    // }
    fn compute(&self, preds: &[f32], targets: &[f32]) -> Loss {
        assert_eq!(
            preds.len(),
            targets.len(),
            "Outputs and targets must have the same length."
        );
        let len = preds.len() as f32;

        // 並列に誤差の二乗と勾配をタプルとして計算し、それぞれの Vec に分離（順序は保たれます）
        let (losses, grads): (Vec<f32>, Vec<f32>) = preds
            .par_iter()
            .zip(targets.par_iter())
            .map(|(&pred, &target)| {
                let error = pred - target;
                (error * error, 2.0 * error)
            })
            .unzip();

        // 合計および平均の計算
        let loss_value = losses.into_iter().sum::<f32>() / len;
        let grad = grads.into_iter().map(|g| g / len).collect();

        Loss {
            value: loss_value,
            grad,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mse_loss() {
        let mse = Mse::new();
        let pred = vec![0.0, 0.5, 1.0];
        let targets = vec![0.0, 1.0, 1.0];

        let loss = mse.compute(&pred, &targets);

        // 損失値を検証
        assert!((loss.value - 0.08333333).abs() < 1e-6);

        // 勾配を検証
        let expected_grad = [0.0, -0.33333334, 0.0];
        for (g, e) in loss.grad.iter().zip(expected_grad.iter()) {
            assert!((g - e).abs() < 1e-6);
        }
    }

    #[test]
    #[should_panic(expected = "Outputs and targets must have the same length.")]
    fn test_mse_length_mismatch() {
        let mse = Mse::new();
        let pred = vec![0.0, 0.5];
        let targets = vec![0.0, 1.0, 1.0];

        mse.compute(&pred, &targets); // パニックを期待
    }
}
