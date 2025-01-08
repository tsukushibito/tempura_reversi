use std::{
    fs::File,
    io::{Read, Write},
};

use crate::{sparse_vector::SparseVector, ResultBoxErr};

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

    pub fn load(file_path: &str) -> ResultBoxErr<Self> {
        let mut file = File::open(file_path)?;
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        let model: Self = bincode::deserialize(&buf)?;

        Ok(model)
    }

    pub fn save(&self, file_path: &str) -> ResultBoxErr<()> {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sparse_vector::SparseVector;

    #[test]
    fn test_forward() {
        let mut model = Model::new(3);
        model.weights[0] = 1.0;
        model.weights[1] = 2.0;
        model.weights[2] = 3.0;

        let input1 = SparseVector::new(vec![0, 1], vec![1.0, 2.0], 3).unwrap();
        let input2 = SparseVector::new(vec![1, 2], vec![3.0, 4.0], 3).unwrap();

        let inputs = vec![input1, input2];

        let outputs = model.forward(&inputs);

        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], 5.0);
        assert_eq!(outputs[1], 18.0);
    }
}
