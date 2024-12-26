mod adam;

pub use adam::Adam;

pub trait Optimizer {
    fn step(&mut self, params: &mut [f32], grads: &[f32]);
    fn set_learning_rate(&mut self, lr: f32);
    fn get_learning_rate(&self) -> f32;
    fn reset(&mut self);
}
