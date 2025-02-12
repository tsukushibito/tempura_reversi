use crate::utils::Feature;

/// Represents the training dataset containing features and their corresponding labels.
#[derive(Debug, Clone)]
pub struct Dataset {
    /// A collection of features, each containing phase and sparse feature vector.
    pub features: Vec<Feature>,
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

    /// Adds a new sample to the dataset.
    pub fn add_sample(&mut self, feature: Feature, label: f32) {
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
