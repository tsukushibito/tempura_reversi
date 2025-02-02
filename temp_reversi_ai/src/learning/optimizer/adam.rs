use crate::utils::SparseVector;

use super::Optimizer;

/// Implementation of Adam optimizer
pub struct Adam {
    learning_rate: f32,
    beta1: f32,
    beta2: f32,
    epsilon: f32,
    m: Vec<f32>, // First moment vector
    v: Vec<f32>, // Second moment vector
    t: usize,    // Time step
}

impl Adam {
    /// Creates a new Adam optimizer
    pub fn new(feature_size: usize, learning_rate: f32) -> Self {
        Self {
            learning_rate,
            beta1: 0.9,
            beta2: 0.999,
            epsilon: 1e-8,
            m: vec![0.0; feature_size],
            v: vec![0.0; feature_size],
            t: 0,
        }
    }
}

impl Optimizer for Adam {
    /// Updates model parameters using sparse gradients
    fn update(
        &mut self,
        weights: &mut [f32],
        bias: &mut f32,
        gradients: &SparseVector,
        bias_grad: f32,
    ) {
        self.t += 1;
        for (&index, &grad) in gradients.indices().iter().zip(gradients.values().iter()) {
            self.m[index] = self.beta1 * self.m[index] + (1.0 - self.beta1) * grad;
            self.v[index] = self.beta2 * self.v[index] + (1.0 - self.beta2) * grad.powi(2);

            let m_hat = self.m[index] / (1.0 - self.beta1.powi(self.t as i32));
            let v_hat = self.v[index] / (1.0 - self.beta2.powi(self.t as i32));

            weights[index] -= self.learning_rate * m_hat / (v_hat.sqrt() + self.epsilon);
        }

        // Update bias term
        *bias -= self.learning_rate * bias_grad;
    }
}
