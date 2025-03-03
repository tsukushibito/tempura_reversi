use std::{
    fs::File,
    io::{Read, Write},
};

use crate::utils::Feature;
use lz4_flex::{compress_prepend_size, decompress_size_prepended};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents the machine learning model
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Model {
    pub weights: Vec<Vec<f32>>,
    pub bias: f32,
}

impl Model {
    /// Predicts outputs for a batch of feature vectors
    pub fn predict(&self, features: &[Feature]) -> Vec<f32> {
        if features.len() == 1 {
            return vec![self.bias + features[0].vector.dot(&self.weights[features[0].phase])];
        } else {
            features
                .par_iter()
                .map(|feature| self.bias + feature.vector.dot(&self.weights[feature.phase]))
                .collect()
        }
    }

    /// Saves the model to a file
    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let serialized = bincode::serialize(self).expect("Failed to serialize model.");
        let compressed = compress_prepend_size(&serialized);
        let mut file = File::create(path)?;
        file.write_all(&compressed)?;
        Ok(())
    }

    /// Loads the model from a file
    pub fn load(path: &str) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let decompressed = decompress_size_prepended(&buffer).expect("Failed to decompress model.");
        let model = bincode::deserialize(&decompressed).expect("Failed to deserialize model.");
        Ok(model)
    }
}
