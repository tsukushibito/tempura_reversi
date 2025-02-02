use std::{
    fs::File,
    io::{Read, Write},
};

use crate::utils::SparseVector;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents the machine learning model
#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub weights: Vec<f32>,
    pub bias: f32,
}

impl Model {
    /// Predicts outputs for a batch of feature vectors
    pub fn predict(&self, features_batch: &[SparseVector]) -> Vec<f32> {
        features_batch
            .par_iter()
            .map(|features| self.bias + features.dot(&self.weights))
            .collect()
    }

    /// Saves the model to a file
    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let serialized = bincode::serialize(self).expect("Failed to serialize model.");
        let mut file = File::create(path)?;
        file.write_all(&serialized)?;
        Ok(())
    }

    /// Loads the model from a file
    pub fn load(path: &str) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let model = bincode::deserialize(&buffer).expect("Failed to deserialize model.");
        Ok(model)
    }
}
