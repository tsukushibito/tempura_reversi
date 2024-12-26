mod cross_entropy;
mod mse;

pub trait LossFunction {
    fn compute(&self, outputs: &[f32], targets: &[f32]) -> Loss;
}

pub struct Loss {
    pub value: f32,     // 損失値
    pub grad: Vec<f32>, // 出力に対する損失の勾配
}
