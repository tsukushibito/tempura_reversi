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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update() {
        // Create an Adam optimizer for 3 features and initial weights and bias.
        let mut adam = Adam::new(3, 0.1);
        let mut weights = vec![0.5, 0.5, 0.5];
        let mut bias = 0.0;

        // Create a SparseVector with one non-zero gradient at index 1.
        let gradients = SparseVector::new(vec![1], vec![0.2], 1);

        // Apply update with a bias gradient of 0.1.
        adam.update(&mut weights, &mut bias, &gradients.unwrap(), 0.1);

        // For t = 1, the expected computations:
        // m[1] = 0.9*0 + 0.1*0.2 = 0.02 and v[1] = 0.999*0 + 0.001*0.04 = 0.00004.
        // m_hat = 0.02 / (1.0 - 0.9) = 0.02 / 0.1 = 0.2.
        // v_hat = 0.00004 / (1.0 - 0.999) = 0.00004 / 0.001 = 0.04.
        // New weight[1] = 0.5 - 0.1*(0.2/(0.2 + 1e-8)) ≈ 0.4.
        // New bias = 0.0 - 0.1*0.1 = -0.01.
        assert!((weights[1] - 0.4).abs() < 1e-6);
        assert!((bias + 0.01).abs() < 1e-6);

        // Unchanged weights.
        assert_eq!(weights[0], 0.5);
        assert_eq!(weights[2], 0.5);
    }
}
