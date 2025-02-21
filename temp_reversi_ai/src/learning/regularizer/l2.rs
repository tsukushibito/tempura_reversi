use super::Regularizer;

pub struct L2Regularizer {
    pub lambda: f32,
}

impl Regularizer for L2Regularizer {
    fn regularize(&self, parameters: &[f32]) -> f32 {
        self.lambda * parameters.iter().map(|x| x * x).sum::<f32>()
    }
}
