use crate::ml::optimizer::Optimizer;

use super::LrScheduler;

#[derive(Debug, Clone)]
pub struct ExponentialLr {
    gamma: f32, // 減衰率
    current_step: usize,
}

impl ExponentialLr {
    pub fn new(gamma: f32) -> Self {
        ExponentialLr {
            gamma,
            current_step: 0,
        }
    }
}

impl LrScheduler for ExponentialLr {
    fn step(&mut self, optimizer: &mut impl Optimizer) {
        self.current_step += 1;
        let old_lr = optimizer.get_learning_rate();
        let new_lr = old_lr * self.gamma;
        optimizer.set_learning_rate(new_lr);
        println!(
            "ExponentialLR: Step {}, learning rate updated from {} to {}",
            self.current_step, old_lr, new_lr
        );
    }
}
