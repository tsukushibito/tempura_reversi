use std::{
    fs::File,
    io::{Read, Write},
};

use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::feature::Feature;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuntimeModel {
    pub weights: Vec<Vec<f32>>,
}

impl RuntimeModel {
    /// Saves the model to a file
    /// Serializes the model using bincode and compresses it with gzip before writing to disk
    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let serialized = bincode::serde::encode_to_vec(self, bincode::config::standard())
            .expect("Failed to serialize model.");

        let file = File::create(path)?;
        let mut encoder = GzEncoder::new(file, Compression::default());
        encoder.write_all(&serialized)?;
        encoder.finish()?;
        Ok(())
    }

    /// Saves the model to a file without compression
    /// Serializes the model using bincode and writes directly to disk
    pub fn save_uncompressed(&self, path: &str) -> std::io::Result<()> {
        let serialized = bincode::serde::encode_to_vec(self, bincode::config::standard())
            .expect("Failed to serialize model.");

        let mut file = File::create(path)?;
        file.write_all(&serialized)?;
        Ok(())
    }

    /// Loads the model from a file
    /// Reads a gzip-compressed file, decompresses it, and deserializes the model using bincode
    pub fn load(path: &str) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let mut decoder = GzDecoder::new(file);
        let mut buffer = Vec::new();
        decoder.read_to_end(&mut buffer)?;

        let (model, _) = bincode::serde::decode_from_slice(&buffer, bincode::config::standard())
            .expect("Failed to deserialize model.");
        Ok(model)
    }

    /// Loads the model from an uncompressed file
    /// Reads the file directly and deserializes the model using bincode
    pub fn load_uncompressed(path: &str) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let (model, _) = bincode::serde::decode_from_slice(&buffer, bincode::config::standard())
            .expect("Failed to deserialize model.");
        Ok(model)
    }

    /// Predicts the evaluation score for a single feature
    /// Uses the weights corresponding to the game phase and sums the weights at the feature indices
    pub fn predict_one(&self, feature: &Feature) -> f32 {
        let w = &self.weights[feature.phase as usize];
        let mut value = 0.0;
        // Sum the weights at each feature index to compute the evaluation score
        for i in 0..feature.indices.len() {
            let index = feature.indices[i] as usize;
            value += w[index];
        }
        value
    }

    /// Predicts evaluation scores for multiple features
    /// For single feature, returns a vector with one prediction
    /// For multiple features, uses parallel processing to compute predictions concurrently
    pub fn predict(&self, features: &[Feature]) -> Vec<f32> {
        if features.len() == 1 {
            vec![self.predict_one(&features[0])]
        } else {
            features
                .par_iter()
                .map(|feature| self.predict_one(feature))
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::patterns::PATTERNS;
    use std::path::Path;

    #[test]
    fn test_predict_one_basic() {
        // Create a simple model with weights for phase 0
        let weights = vec![
            vec![1.0, 2.0, 3.0, 4.0, 5.0], // weights for phase 0
            vec![0.5, 1.5, 2.5, 3.5, 4.5], // weights for phase 1
        ];
        let model = RuntimeModel { weights };

        // Create a feature with some indices
        let mut feature = Feature::default();
        feature.phase = 0;
        feature.indices[0] = 0; // first pattern index 0 -> weight 1.0
        feature.indices[1] = 2; // second pattern index 2 -> weight 3.0
        feature.indices[2] = 4; // third pattern index 4 -> weight 5.0
                                // remaining indices are 0, so they contribute weight 1.0 each

        let result = model.predict_one(&feature);

        // Expected: 1.0 + 3.0 + 5.0 + (PATTERNS.len() - 3) * 1.0
        let expected = 1.0 + 3.0 + 5.0 + (PATTERNS.len() - 3) as f32 * 1.0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_predict_one_different_phase() {
        let weights = vec![vec![1.0, 2.0, 3.0], vec![10.0, 20.0, 30.0]];
        let model = RuntimeModel { weights };

        let mut feature = Feature::default();
        feature.phase = 1; // Use phase 1 weights
        feature.indices[0] = 0; // index 0 -> weight 10.0
        feature.indices[1] = 2; // index 2 -> weight 30.0

        let result = model.predict_one(&feature);

        // Expected: 10.0 + 30.0 + (PATTERNS.len() - 2) * 10.0 (remaining indices are 0)
        let expected = 10.0 + 30.0 + (PATTERNS.len() - 2) as f32 * 10.0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_predict_one_zero_weights() {
        let weights = vec![vec![0.0; 100]]; // All weights are 0
        let model = RuntimeModel { weights };

        let feature = Feature::default(); // All indices are 0, phase is 0

        let result = model.predict_one(&feature);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_predict_one_negative_weights() {
        let weights = vec![vec![-1.0, -2.0, -3.0, -4.0, -5.0]];
        let model = RuntimeModel { weights };

        let mut feature = Feature::default();
        feature.indices[0] = 1; // index 1 -> weight -2.0
        feature.indices[1] = 3; // index 3 -> weight -4.0
                                // remaining indices are 0 -> weight -1.0 each

        let result = model.predict_one(&feature);

        let expected = -2.0 + -4.0 + (PATTERNS.len() - 2) as f32 * -1.0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_load_model() {
        let test_file_path = "../work/artifacts/test_runtime_model.gz";

        // Verify the test file exists
        assert!(
            Path::new(test_file_path).exists(),
            "Test file {} does not exist. Please ensure the file is available for testing.",
            test_file_path
        );

        // Load the model
        let loaded_model =
            RuntimeModel::load(test_file_path).expect("Failed to load model from test file");

        // Verify the model structure
        assert!(
            !loaded_model.weights.is_empty(),
            "Model should have weights"
        );

        // Verify we have the expected number of phases
        assert!(
            loaded_model.weights.len() <= 65, // PHASE_COUNT is 65
            "Model should not have more than 65 phases, got {}",
            loaded_model.weights.len()
        );

        // Verify each phase has weights
        for (phase_idx, phase_weights) in loaded_model.weights.iter().enumerate() {
            assert!(
                !phase_weights.is_empty(),
                "Phase {} should have non-empty weights",
                phase_idx
            );
        }

        // Test that the model can make predictions
        let test_feature = Feature::default();
        let prediction = loaded_model.predict_one(&test_feature);

        // The prediction should be a finite number
        assert!(
            prediction.is_finite(),
            "Model prediction should be finite, got {}",
            prediction
        );
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        use tempfile::NamedTempFile;

        // Create a test model
        let original_weights = vec![
            vec![1.0, -2.5, 3.14, 0.0, -1.5],
            vec![0.5, 2.0, -0.75, 1.25, -3.0],
        ];
        let original_model = RuntimeModel {
            weights: original_weights.clone(),
        };

        // Create temporary file
        let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
        let temp_path = temp_file.path().to_str().unwrap();

        original_model
            .save(temp_path)
            .expect("Failed to save model");

        // Load the model back
        let loaded_model = RuntimeModel::load(temp_path).expect("Failed to load model");

        // File will be automatically cleaned up when temp_file goes out of scope

        // Verify the loaded model matches the original
        assert_eq!(loaded_model.weights.len(), original_model.weights.len());

        for (phase_idx, (original_phase, loaded_phase)) in original_model
            .weights
            .iter()
            .zip(loaded_model.weights.iter())
            .enumerate()
        {
            assert_eq!(
                original_phase.len(),
                loaded_phase.len(),
                "Phase {} weight count mismatch",
                phase_idx
            );

            for (weight_idx, (original_weight, loaded_weight)) in
                original_phase.iter().zip(loaded_phase.iter()).enumerate()
            {
                assert!(
                    (original_weight - loaded_weight).abs() < 1e-6,
                    "Weight mismatch at phase {} index {}: original={}, loaded={}",
                    phase_idx,
                    weight_idx,
                    original_weight,
                    loaded_weight
                );
            }
        }

        // Test that both models produce the same predictions
        let test_feature = Feature::default();
        let original_prediction = original_model.predict_one(&test_feature);
        let loaded_prediction = loaded_model.predict_one(&test_feature);

        assert!(
            (original_prediction - loaded_prediction).abs() < 1e-6,
            "Prediction mismatch: original={}, loaded={}",
            original_prediction,
            loaded_prediction
        );
    }

    #[test]
    fn test_load_nonexistent_file() {
        let result = RuntimeModel::load("nonexistent_file.gz");
        assert!(
            result.is_err(),
            "Loading nonexistent file should return an error"
        );
    }

    #[test]
    fn test_load_invalid_file() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        // Create a temporary file with invalid content
        let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
        temp_file
            .write_all(b"invalid content")
            .expect("Failed to write test data");

        let temp_path = temp_file.path().to_str().unwrap();
        let result = RuntimeModel::load(temp_path);

        // File will be automatically cleaned up when temp_file goes out of scope

        assert!(
            result.is_err(),
            "Loading invalid file should return an error"
        );
    }
}
