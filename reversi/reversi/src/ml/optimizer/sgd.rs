use super::Optimizer;
use crate::SparseVector;

#[derive(Debug, Default, Clone)]
pub struct Sgd {
    learning_rate: f32,
}

impl Sgd {
    pub fn new(learning_rate: f32) -> Self {
        Sgd { learning_rate }
    }
}

impl Optimizer for Sgd {
    fn step(&mut self, params: &mut [f32], grads: &SparseVector) {
        grads.iter().for_each(|(i, g)| {
            params[i] -= self.learning_rate * g;
        });
    }

    fn set_learning_rate(&mut self, lr: f32) {
        self.learning_rate = lr;
    }

    fn get_learning_rate(&self) -> f32 {
        self.learning_rate
    }

    fn reset(&mut self) {
        // SGDでは特にリセットする状態はないが、メソッドを用意しておく
    }
}

#[cfg(test)]
mod tests {
    use crate::ResultBoxErr;

    use super::*;

    #[test]
    fn test_sgd_step() -> ResultBoxErr<()> {
        let mut optimizer = Sgd::new(0.1);
        let mut params = vec![1.0, 2.0, 3.0];
        // let grads = SparseVector::new(vec![0, 1, 2], vec![0.5, 0.2, 0.1], 3)?;
        let grads = SparseVector::from(&[(0, 0.5), (1, 0.2), (2, 0.1)], 3)?;

        optimizer.step(&mut params, &grads);

        assert_eq!(params, vec![0.95, 1.98, 2.99]);

        Ok(())
    }

    #[test]
    fn test_sgd_learning_rate() {
        let mut optimizer = Sgd::new(0.1);

        assert_eq!(optimizer.get_learning_rate(), 0.1);

        optimizer.set_learning_rate(0.01);
        assert_eq!(optimizer.get_learning_rate(), 0.01);
    }

    #[test]
    fn test_sgd_reset() {
        let mut optimizer = Sgd::new(0.1);
        optimizer.reset(); // 確認する状態はないが、エラーなく呼び出せることを確認
    }
}
