mod exponential_lr;
mod step_lr;

pub use exponential_lr::ExponentialLR;
pub use step_lr::StepLR;

use super::optimizer::Optimizer;

pub trait LRScheduler {
    fn step(&mut self, optimizer: &mut dyn Optimizer);
}
