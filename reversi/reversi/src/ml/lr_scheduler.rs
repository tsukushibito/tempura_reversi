mod exponential_lr;
mod step_lr;

pub use exponential_lr::ExponentialLr;
pub use step_lr::StepLr;

use super::optimizer::Optimizer;

pub trait LrScheduler: Clone {
    fn step(&mut self, optimizer: &mut impl Optimizer);
}
