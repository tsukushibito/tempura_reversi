use std::{
    fs::File,
    io::{Read, Write},
};

use lz4_flex::{compress_prepend_size, decompress_size_prepended};
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
        let (model, _) =
            bincode::serde::decode_from_slice(&decompressed, bincode::config::standard())
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
