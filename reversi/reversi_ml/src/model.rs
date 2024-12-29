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

    pub fn forward(&self, input: SparseVector) -> f32 {
        input.dot(&self.weights).unwrap()
    }

    pub fn backward(&mut self, grad_output: f32, input: &SparseVector) -> Gradients {
        let mut grad_weights = SparseVector::new(vec![], vec![], self.weights.len()).unwrap();

        for (index, value) in input.iter() {
            let grad = grad_output * value;
            grad_weights.assign(index, grad);
        }

        Gradients {
            weights: grad_weights,
        }
    }
}
