use std::collections::HashMap;

use super::Optimizer;

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
    fn step(&mut self, params: &mut [f32], grads: &[f32]) {
        self.t += 1;
        for (i, (&p, &g)) in params.iter_mut().zip(grads.iter()).enumerate() {
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
            *p -= self.learning_rate * m_hat / (v_hat.sqrt() + self.epsilon);
        }
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
