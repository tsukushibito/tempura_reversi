mod exponential_lr;
mod step_lr;

pub use step_lr::StepLR;

use crate::optimizer::Optimizer;

pub trait LRScheduler {
    fn step(&mut self, optimizer: &mut dyn Optimizer);
}
