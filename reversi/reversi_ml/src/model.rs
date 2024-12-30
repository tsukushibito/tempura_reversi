use std::{
    fs::File,
    io::{Read, Write},
};

use crate::{sparse_vector::SparseVector, DynResult};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Model {
    pub weights: Vec<f32>,
}

#[derive(Debug)]
pub struct Gradients {
    pub weights: SparseVector,
}

impl Model {
    pub fn new(input_size: usize) -> Self {
        let weights = (0..input_size)
            .map(|_| rand::random::<f32>() * 0.01)
            .collect();
        Self { weights }
    }

    pub fn load(file_path: &str) -> DynResult<Self> {
        let mut file = File::open(file_path)?;
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        let model: Self = bincode::deserialize(&buf)?;

        Ok(model)
    }

    pub fn save(&self, file_path: &str) -> DynResult<()> {
        let mut file = File::open(file_path)?;
        let serialized = bincode::serialize(self)?;
        file.write_all(&serialized)?;
        file.flush()?;
        Ok(())
    }

    pub fn forward(&self, inputs: &[SparseVector]) -> Vec<f32> {
        inputs
            .iter()
            .map(|input| input.dot(&self.weights).unwrap())
            .collect()
    }

    pub fn backward(&mut self, grad_outputs: &[f32], inputs: &[SparseVector]) -> Gradients {
        let mut grad_weights = grad_outputs
            .iter()
            .zip(inputs.iter())
            .map(|(&grad_output, input)| input.clone() * grad_output)
            .reduce(|g1, g2| g1 + g2)
            .unwrap();

        grad_weights = grad_weights / grad_outputs.len() as f32;

        Gradients {
            weights: grad_weights,
        }
    }
}
