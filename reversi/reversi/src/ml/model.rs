use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{sparse_vector::SparseVector, ResultBoxErr};

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Model {
    pub params: Vec<Vec<f32>>,
}

pub struct ModelInput {
    pub phase: usize,
    pub feature: SparseVector,
}

impl Model {
    pub fn new(feature_size: usize) -> Self {
        let params = (0..60)
            .map(|_| {
                (0..feature_size)
                    .map(|_| rand::random::<f32>() * 0.01)
                    .collect()
            })
            .collect();

        Self { params }
    }

    pub fn load_model<P: AsRef<Path>>(file_path: P) -> ResultBoxErr<Self> {
        let mut file = File::open(file_path)?;
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        let model: Self = bincode::deserialize(&buf)?;

        Ok(model)
    }

    pub fn save_model<P: AsRef<Path>>(model: &Model, file_path: P) -> ResultBoxErr<()> {
        let mut file = File::create(file_path)?;
        let serialized = bincode::serialize(model)?;
        file.write_all(&serialized)?;
        file.flush()?;
        Ok(())
    }

    pub fn forward(&self, inputs: &[ModelInput]) -> Vec<f32> {
        inputs
            .par_iter()
            .map(|input| input.feature.dot(&self.params[input.phase]).unwrap())
            .collect()
    }
}

pub fn load_models<P: AsRef<Path>>(file_path: P) -> ResultBoxErr<Vec<Model>> {
    let mut file = File::open(file_path)?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let models: Vec<Model> = bincode::deserialize(&buf)?;

    Ok(models)
}

pub fn save_models<P: AsRef<Path>>(models: &Vec<Model>, file_path: P) -> ResultBoxErr<()> {
    let mut file = File::create(file_path)?;
    let serialized = bincode::serialize(models)?;
    file.write_all(&serialized)?;
    file.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sparse_vector::SparseVector;

    #[test]
    fn test_forward() {}
}
