use super::SparseVector;

/// Represents a single input feature containing phase and sparse feature vector.
#[derive(Debug, Clone, Default)]
pub struct Feature {
    /// The phase (e.g. move number or game stage) of the sample.
    pub phase: usize,
    /// The sparse feature vector of the sample.
    pub vector: SparseVector,
}
