/// Trait for regularization methods.
pub trait Regularizer {
    /// Computes the regularization value for the given parameters.
    fn regularize(&self, parameters: &[f32]) -> f32;
}

mod elastic_net;
mod l1;
mod l2;

pub use elastic_net::*;
pub use l1::*;
pub use l2::*;
