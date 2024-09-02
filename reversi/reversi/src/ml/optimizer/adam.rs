use std::collections::HashMap;

use crate::sparse_vector::SparseVector;

use super::Optimizer;

#[derive(Debug, Default, Clone)]
pub struct Adam {
    learning_rate: f32,
    beta1: f32,
    beta2: f32,
    epsilon: f32,
    m: HashMap<usize, f32>,
    v: HashMap<usize, f32>,
    t: usize,
}

impl Adam {
    pub fn new(learning_rate: f32, beta1: f32, beta2: f32, epsilon: f32) -> Self {
        Adam {
            learning_rate,
            beta1,
            beta2,
            epsilon,
            m: HashMap::new(),
            v: HashMap::new(),
            t: 0,
        }
    }
}

impl Optimizer for Adam {
    fn step(&mut self, params: &mut [f32], grads: &SparseVector) {
        self.t += 1;
        grads.iter().for_each(|(i, g)| {
            // 第1モーメントの更新
            let m = self.m.entry(i).or_insert(0.0);
            *m = self.beta1 * (*m) + (1.0 - self.beta1) * g;

            // 第2モーメントの更新
            let v = self.v.entry(i).or_insert(0.0);
            *v = self.beta2 * (*v) + (1.0 - self.beta2) * g * g;

            // バイアス補正
            let m_hat = *m / (1.0 - self.beta1.powi(self.t as i32));
            let v_hat = *v / (1.0 - self.beta2.powi(self.t as i32));

            // パラメータの更新
            params[i] -= self.learning_rate * m_hat / (v_hat.sqrt() + self.epsilon);
        });
    }

    fn set_learning_rate(&mut self, lr: f32) {
        self.learning_rate = lr;
    }

    fn get_learning_rate(&self) -> f32 {
        self.learning_rate
    }

    fn reset(&mut self) {
        self.m.clear();
        self.v.clear();
        self.t = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{sparse_vector::SparseVector, ResultBoxErr};

    #[test]
    fn test_adam_step() -> ResultBoxErr<()> {
        let mut optimizer = Adam::new(0.001, 0.9, 0.999, 1e-8);
        let mut params = vec![1.0, 2.0, 3.0];
        let grads = SparseVector::from(&[(0, 0.5), (1, 0.2), (2, 0.1)], 3)?;

        optimizer.step(&mut params, &grads);

        // 確実な数値をチェックするには計算結果に基づいた期待値が必要です。
        // とりあえず結果が適切に更新されていることを確認します。
        assert!(params[0] < 1.0);
        assert!(params[1] < 2.0);
        assert!(params[2] < 3.0);

        Ok(())
    }

    #[test]
    fn test_adam_learning_rate() {
        let mut optimizer = Adam::new(0.001, 0.9, 0.999, 1e-8);

        assert_eq!(optimizer.get_learning_rate(), 0.001);

        optimizer.set_learning_rate(0.01);
        assert_eq!(optimizer.get_learning_rate(), 0.01);
    }

    #[test]
    fn test_adam_reset() -> ResultBoxErr<()> {
        let mut optimizer = Adam::new(0.001, 0.9, 0.999, 1e-8);
        let grads = SparseVector::from(&[(0, 0.5), (1, 0.2), (2, 0.1)], 3)?;
        let mut params = vec![1.0, 2.0, 3.0];

        optimizer.step(&mut params, &grads);
        optimizer.reset();

        assert!(optimizer.m.is_empty());
        assert!(optimizer.v.is_empty());
        assert_eq!(optimizer.t, 0);

        Ok(())
    }
}
