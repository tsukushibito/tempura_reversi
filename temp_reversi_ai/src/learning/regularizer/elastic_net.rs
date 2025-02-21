use super::Regularizer;

pub struct ElasticNetRegularizer {
    pub lambda_l1: f32,
    pub lambda_l2: f32,
}

impl Regularizer for ElasticNetRegularizer {
    fn regularize(&self, parameters: &[f32]) -> f32 {
        let l1 = parameters.iter().map(|x| x.abs()).sum::<f32>();
        let l2 = parameters.iter().map(|x| x * x).sum::<f32>();
        self.lambda_l1 * l1 + self.lambda_l2 * l2
    }
}
