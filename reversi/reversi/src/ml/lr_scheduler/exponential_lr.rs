use crate::ml::optimizer::Optimizer;

use super::LRScheduler;

pub struct ExponentialLR {
    gamma: f32, // 減衰率
    current_step: usize,
}

impl ExponentialLR {
    pub fn new(gamma: f32) -> Self {
        ExponentialLR {
            gamma,
            current_step: 0,
        }
    }
}

impl LRScheduler for ExponentialLR {
    fn step(&mut self, optimizer: &mut dyn Optimizer) {
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
