use crate::ml::optimizer::Optimizer;

use super::LRScheduler;

pub struct StepLR {
    step_size: usize, // 学習率を減衰させるステップ数
    gamma: f32,       // 減衰率
    current_step: usize,
}

impl StepLR {
    pub fn new(step_size: usize, gamma: f32) -> Self {
        StepLR {
            step_size,
            gamma,
            current_step: 0,
        }
    }
}

impl LRScheduler for StepLR {
    fn step(&mut self, optimizer: &mut dyn Optimizer) {
        self.current_step += 1;
        if self.current_step % self.step_size == 0 {
            let old_lr = optimizer.get_learning_rate();
            let new_lr = old_lr * self.gamma;
            optimizer.set_learning_rate(new_lr);
            println!(
                "StepLR: Step {}, learning rate updated from {} to {}",
                self.current_step, old_lr, new_lr
            );
        }
    }
}