use rayon::prelude::*;

use crate::Model;

use super::optimizer::Optimizer;

#[derive(Debug)]
pub struct AdamW {
    lr: f32,
    beta1: f32,
    beta2: f32,
    epsilon: f32,
    weight_decay: f32,
    t: usize,
    m_weights: Vec<f32>,
    v_weights: Vec<f32>,
    m_biases: Vec<f32>,
    v_biases: Vec<f32>,
}

impl AdamW {
    pub fn new(
        weight_dim: usize,
        bias_dim: usize,
        lr: f32,
        beta1: f32,
        beta2: f32,
        epsilon: f32,
        weight_decay: f32,
    ) -> Self {
        AdamW {
            lr,
            beta1,
            beta2,
            epsilon,
            weight_decay,
            t: 0,
            m_weights: vec![0.0; weight_dim],
            v_weights: vec![0.0; weight_dim],
            m_biases: vec![0.0; bias_dim],
            v_biases: vec![0.0; bias_dim],
        }
    }
}

impl Optimizer for AdamW {
    fn update(&mut self, model: &mut impl Model, weight_grads: &[f32], bias_grads: &[f32]) {
        self.t += 1;
        let t = self.t as f32;
        let beta1 = self.beta1;
        let beta2 = self.beta2;
        let lr = self.lr;
        let epsilon = self.epsilon;
        let weight_decay = self.weight_decay;
        let (weights, biases) = model.parameterss_mut();

        weights
            .par_iter_mut()
            .zip(self.m_weights.par_iter_mut())
            .zip(self.v_weights.par_iter_mut())
            .zip(weight_grads.par_iter())
            .for_each(|(((weight, m_i), v_i), &grad)| {
                *m_i = beta1 * (*m_i) + (1.0 - beta1) * grad;
                *v_i = beta2 * (*v_i) + (1.0 - beta2) * grad.powi(2);

                let m_hat = *m_i / (1.0 - beta1.powf(t));
                let v_hat = *v_i / (1.0 - beta2.powf(t));

                *weight -= lr * (m_hat / (v_hat.sqrt() + epsilon));
                *weight -= lr * weight_decay * *weight;
            });

        biases
            .par_iter_mut()
            .zip(self.m_biases.par_iter_mut())
            .zip(self.v_biases.par_iter_mut())
            .zip(bias_grads.par_iter())
            .for_each(|(((bias, m_i), v_i), &grad)| {
                *m_i = beta1 * (*m_i) + (1.0 - beta1) * grad;
                *v_i = beta2 * (*v_i) + (1.0 - beta2) * grad.powi(2);

                let m_hat = *m_i / (1.0 - beta1.powf(t));
                let v_hat = *v_i / (1.0 - beta2.powf(t));

                *bias -= lr * (m_hat / (v_hat.sqrt() + epsilon));
            });
    }
}
