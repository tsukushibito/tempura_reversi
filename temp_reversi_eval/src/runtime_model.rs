use std::{
    fs::File,
    io::{Read, Write},
};

use flate2::{
    write::{GzDecoder, GzEncoder},
    Compression,
};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::feature::Feature;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuntimeModel {
    pub weights: Vec<Vec<f32>>,
}

impl RuntimeModel {
    /// Saves the model to a file
    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let serialized = bincode::serde::encode_to_vec(self, bincode::config::standard())
            .expect("Failed to serialize model.");

        let file = File::create(path)?;
        let mut encoder = GzEncoder::new(file, Compression::default());
        encoder.write_all(&serialized)?;
        encoder.finish()?;
        Ok(())
    }

    /// Loads the model from a file
    pub fn load(path: &str) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let mut decoder = GzDecoder::new(file);
        let mut buffer = Vec::new();
        decoder.read_to_end(&mut buffer)?;

        let (model, _) = bincode::serde::decode_from_slice(&buffer, bincode::config::standard())
            .expect("Failed to deserialize model.");
        Ok(model)
    }

    pub fn predict_one(&self, feature: &Feature) -> f32 {
        let w = &self.weights[feature.phase as usize];
        let mut value = 0.0;
        for i in 0..feature.indices.len() {
            let index = feature.indices[i] as usize;
            value += w[index];
        }
        value
    }

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
