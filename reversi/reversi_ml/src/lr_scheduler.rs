mod exponential_lr;
mod step_lr;

pub use exponential_lr::ExponentialLR;
pub use step_lr::StepLR;

pub trait LRScheduler {
    fn step(&mut self, now_lr: f32) -> f32;
}
