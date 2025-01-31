use crate::utils::SparseVector;

/// A dataset structure for storing feature vectors and their corresponding labels.
#[derive(Debug, Clone)]
pub struct Dataset {
    /// A collection of sparse feature vectors.
    pub features: Vec<SparseVector>,
    /// A collection of labels representing evaluation values (ground truth).
    pub labels: Vec<f32>,
}

impl Dataset {
    /// Creates a new, empty dataset.
    ///
    /// # Returns
    ///
    /// A new `Dataset` instance with empty feature and label vectors.
    ///
    /// # Example
    ///
    /// ```
    /// let dataset = Dataset::new();
    /// assert!(dataset.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            features: Vec::new(),
            labels: Vec::new(),
        }
    }

    /// Adds a sample consisting of a feature vector and a label to the dataset.
    ///
    /// # Arguments
    ///
    /// * `feature` - A `SparseVector` representing the feature vector.
    /// * `label` - A `f32` value representing the corresponding evaluation.
    ///
    /// # Example
    ///
    /// ```
    /// let mut dataset = Dataset::new();
    /// let feature = SparseVector::new();
    /// dataset.add_sample(feature, 1.5);
    /// assert_eq!(dataset.len(), 1);
    /// ```
    pub fn add_sample(&mut self, feature: SparseVector, label: f32) {
        self.features.push(feature);
        self.labels.push(label);
    }

    /// Returns the number of samples in the dataset.
    ///
    /// # Returns
    ///
    /// * A `usize` value representing the number of stored samples.
    ///
    /// # Example
    ///
    /// ```
    /// let dataset = Dataset::new();
    /// assert_eq!(dataset.len(), 0);
    /// ```
    pub fn len(&self) -> usize {
        self.features.len()
    }

    /// Checks if the dataset is empty.
    ///
    /// # Returns
    ///
    /// * `true` if the dataset contains no samples, otherwise `false`.
    ///
    /// # Example
    ///
    /// ```
    /// let dataset = Dataset::new();
    /// assert!(dataset.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.features.is_empty()
    }
}
