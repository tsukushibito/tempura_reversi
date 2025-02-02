use super::Optimizer;

/// Implementation of Adam optimizer
pub struct Adam {
    learning_rate: f32,
    beta1: f32,
    beta2: f32,
    epsilon: f32,
    m: Vec<f32>,
    v: Vec<f32>,
    t: usize,
}

impl Adam {
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
    fn update(&mut self, weights: &mut [f32], bias: &mut f32, gradients: &[f32], bias_grad: f32) {
        self.t += 1;
        for i in 0..weights.len() {
            let grad = gradients[i];

            self.m[i] = self.beta1 * self.m[i] + (1.0 - self.beta1) * grad;
            self.v[i] = self.beta2 * self.v[i] + (1.0 - self.beta2) * grad.powi(2);

            let m_hat = self.m[i] / (1.0 - self.beta1.powi(self.t as i32));
            let v_hat = self.v[i] / (1.0 - self.beta2.powi(self.t as i32));

            weights[i] -= self.learning_rate * m_hat / (v_hat.sqrt() + self.epsilon);
        }

        *bias -= self.learning_rate * bias_grad;
    }
}
