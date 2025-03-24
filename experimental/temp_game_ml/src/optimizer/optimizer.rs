use crate::Model;

pub trait Optimizer {
    fn update(&mut self, model: &mut impl Model, weight_grads: &[f32], bias_grads: &[f32]);
}
