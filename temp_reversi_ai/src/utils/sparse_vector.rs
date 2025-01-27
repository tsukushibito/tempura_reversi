/// Represents a sparse vector with indices and corresponding values.
#[derive(Debug, Clone, PartialEq)]
pub struct SparseVector {
    indices: Vec<usize>,
    values: Vec<f32>,
    size: usize, // Total size of the vector (including implicit zeros)
}

impl SparseVector {
    /// Creates a new sparse vector.
    ///
    /// # Arguments
    /// * `indices` - The indices of non-zero elements.
    /// * `values` - The values of the non-zero elements.
    /// * `size` - The total size of the vector (including implicit zeros).
    ///
    /// # Returns
    /// Returns `Some(SparseVector)` if the input is valid, otherwise `None`.
    ///
    /// # Examples
    /// ```
    /// let indices = vec![0, 3, 7];
    /// let values = vec![1.0, 2.5, -3.0];
    /// let size = 10;
    /// let vector = SparseVector::new(indices, values, size).unwrap();
    /// ```
    pub fn new(indices: Vec<usize>, values: Vec<f32>, size: usize) -> Option<Self> {
        if indices.len() != values.len() || size == 0 {
            return None;
        }
        Some(Self {
            indices,
            values,
            size,
        })
    }

    /// Returns the indices of non-zero elements.
    pub fn indices(&self) -> &[usize] {
        &self.indices
    }

    /// Returns the values of non-zero elements.
    pub fn values(&self) -> &[f32] {
        &self.values
    }

    /// Returns the total size of the vector.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Converts the sparse vector into a dense representation.
    ///
    /// # Returns
    /// A `Vec<f32>` containing all elements, with zeros in positions of implicit elements.
    ///
    /// # Examples
    /// ```
    /// let indices = vec![0, 3, 7];
    /// let values = vec![1.0, 2.5, -3.0];
    /// let size = 10;
    /// let vector = SparseVector::new(indices, values, size).unwrap();
    /// assert_eq!(vector.to_dense(), vec![1.0, 0.0, 0.0, 2.5, 0.0, 0.0, 0.0, -3.0, 0.0, 0.0]);
    /// ```
    pub fn to_dense(&self) -> Vec<f32> {
        let mut dense = vec![0.0; self.size];
        for (&index, &value) in self.indices.iter().zip(self.values.iter()) {
            dense[index] = value;
        }
        dense
    }

    /// Creates a sparse vector from a dense representation.
    ///
    /// # Arguments
    /// * `dense` - A dense vector representation.
    ///
    /// # Returns
    /// A `SparseVector`.
    ///
    /// # Examples
    /// ```
    /// let dense = vec![1.0, 0.0, 0.0, 2.5, 0.0, 0.0, 0.0, -3.0, 0.0, 0.0];
    /// let vector = SparseVector::from_dense(&dense);
    /// assert_eq!(vector.indices(), &[0, 3, 7]);
    /// assert_eq!(vector.values(), &[1.0, 2.5, -3.0]);
    /// assert_eq!(vector.size(), 10);
    /// ```
    pub fn from_dense(dense: &[f32]) -> Self {
        let mut indices = Vec::new();
        let mut values = Vec::new();

        for (i, &value) in dense.iter().enumerate() {
            if value != 0.0 {
                indices.push(i);
                values.push(value);
            }
        }

        Self {
            indices,
            values,
            size: dense.len(),
        }
    }

    /// Computes the dot product of the sparse vector with a dense vector.
    ///
    /// # Arguments
    /// * `dense` - A dense vector to compute the dot product with.
    ///
    /// # Returns
    /// The dot product as a `f32` value.
    ///
    /// # Examples
    /// ```
    /// let sparse = SparseVector::new(vec![0, 2, 4], vec![1.0, -2.0, 3.0], 5).unwrap();
    /// let dense = vec![2.0, 0.0, -1.0, 0.0, 4.0];
    /// assert_eq!(sparse.dot(&dense), 1.0 + (-2.0 * -1.0) + (3.0 * 4.0));
    /// ```
    pub fn dot(&self, dense: &[f32]) -> f32 {
        if dense.len() != self.size {
            panic!(
                "Size mismatch: SparseVector size is {} but dense vector size is {}",
                self.size,
                dense.len()
            );
        }

        self.indices
            .iter()
            .zip(self.values.iter())
            .map(|(&index, &value)| value * dense[index])
            .sum()
    }
}

impl Default for SparseVector {
    /// Creates an empty sparse vector with size 0.
    fn default() -> Self {
        Self {
            indices: Vec::new(),
            values: Vec::new(),
            size: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparse_vector_creation() {
        let indices = vec![0, 3, 7];
        let values = vec![1.0, 2.5, -3.0];
        let size = 10;
        let vector = SparseVector::new(indices.clone(), values.clone(), size).unwrap();

        assert_eq!(vector.indices(), &indices);
        assert_eq!(vector.values(), &values);
        assert_eq!(vector.size(), size);
    }

    #[test]
    fn test_sparse_vector_to_dense() {
        let indices = vec![0, 3, 7];
        let values = vec![1.0, 2.5, -3.0];
        let size = 10;
        let vector = SparseVector::new(indices, values, size).unwrap();

        assert_eq!(
            vector.to_dense(),
            vec![1.0, 0.0, 0.0, 2.5, 0.0, 0.0, 0.0, -3.0, 0.0, 0.0]
        );
    }

    #[test]
    fn test_sparse_vector_from_dense() {
        let dense = vec![1.0, 0.0, 0.0, 2.5, 0.0, 0.0, 0.0, -3.0, 0.0, 0.0];
        let vector = SparseVector::from_dense(&dense);

        assert_eq!(vector.indices(), &[0, 3, 7]);
        assert_eq!(vector.values(), &[1.0, 2.5, -3.0]);
        assert_eq!(vector.size(), 10);
    }

    #[test]
    fn test_sparse_vector_default() {
        let vector = SparseVector::default();
        assert_eq!(vector.indices().len(), 0);
        assert_eq!(vector.values().len(), 0);
        assert_eq!(vector.size(), 0);
    }

    #[test]
    fn test_sparse_vector_dot() {
        let sparse = SparseVector::new(vec![0, 2, 4], vec![1.0, -2.0, 3.0], 5).unwrap();
        let dense = vec![2.0, 0.0, -1.0, 0.0, 4.0];
        assert_eq!(sparse.dot(&dense), 1.0 * 2.0 + (-2.0 * -1.0) + (3.0 * 4.0));
    }

    #[test]
    #[should_panic(expected = "Size mismatch")]
    fn test_sparse_vector_dot_size_mismatch() {
        let sparse = SparseVector::new(vec![0, 2, 4], vec![1.0, -2.0, 3.0], 5).unwrap();
        let dense = vec![2.0, 0.0, -1.0]; // Mismatched size
        sparse.dot(&dense);
    }
}
