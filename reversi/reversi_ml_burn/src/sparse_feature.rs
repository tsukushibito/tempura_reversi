#[derive(Debug, Clone)]
pub struct SparseFeature {
    indices: Vec<usize>, // 非ゼロ要素のインデックス
    values: Vec<f32>,    // 非ゼロ要素の値
}

impl SparseFeature {
    pub fn new(indices: Vec<usize>, values: Vec<f32>) -> Self {
        assert_eq!(
            indices.len(),
            values.len(),
            "Indices and values must have the same length"
        );
        SparseFeature { indices, values }
    }

    pub fn linear_combination(&self, weights: &[f32]) -> f32 {
        self.indices
            .iter()
            .zip(&self.values)
            .map(|(&i, &v)| weights[i] * v)
            .sum()
    }
}
