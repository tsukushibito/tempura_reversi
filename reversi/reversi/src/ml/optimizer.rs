mod adam;

pub use adam::*;

use crate::SparseVector;

pub trait Optimizer {
    fn step(&mut self, params: &mut [f32], grads: &SparseVector);
    fn set_learning_rate(&mut self, lr: f32);
    fn get_learning_rate(&self) -> f32;
    fn reset(&mut self);
}