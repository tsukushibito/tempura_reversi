use super::Regularizer;

pub struct L1Regularizer {
    pub lambda: f32,
}

impl Regularizer for L1Regularizer {
    fn regularize(&self, parameters: &[f32]) -> f32 {
        self.lambda * parameters.iter().map(|x| x.abs()).sum::<f32>()
    }
}
