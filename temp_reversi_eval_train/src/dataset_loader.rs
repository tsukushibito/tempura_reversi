use std::{fs::File, io::copy, path::Path};

use burn::data::dataset::{Dataset, SqliteDataset};
use flate2::read::GzDecoder;
use tempfile::NamedTempFile;

use crate::dataset::{ReversiDataset, ReversiSample};

type BoxError = Box<dyn std::error::Error>;

/// Dataset loader for compressed SQLite datasets
pub struct DatasetLoader {
    pub train_dataset: ReversiDataset,
    pub valid_dataset: ReversiDataset,
}

impl DatasetLoader {
    /// Loads datasets from a compressed SQLite file
    ///
    /// # Arguments
    ///
    /// * `dataset_dir` - Directory containing the compressed dataset file
    /// * `dataset_name` - Name of the dataset file (without .gz extension)
    ///
    /// # Returns
    ///
    /// A DatasetLoader containing both training and validation datasets
    pub fn load_from_compressed(dataset_dir: &str, dataset_name: &str) -> Result<Self, BoxError> {
        // Decompress the .gz file to a temporary file
        let temp_db = Self::decompress_dataset(dataset_dir, dataset_name)?;

        // Load samples from the decompressed database
        let (train_samples, valid_samples) = Self::load_samples_from_db(temp_db.path())?;

        // Create datasets
        let train_dataset = ReversiDataset::new(train_samples);
        let valid_dataset = ReversiDataset::new(valid_samples);

        Ok(DatasetLoader {
            train_dataset,
            valid_dataset,
        })
    }

    /// Decompresses a .gz dataset file to a temporary file
    fn decompress_dataset(
        dataset_dir: &str,
        dataset_name: &str,
    ) -> Result<NamedTempFile, BoxError> {
        let gz_path = Path::new(dataset_dir).join(format!("{}.gz", dataset_name));
        let gz_file = File::open(&gz_path)?;
        let mut decoder = GzDecoder::new(gz_file);

        let temp_file = NamedTempFile::new()?;
        let mut temp_writer = File::create(temp_file.path())?;

        copy(&mut decoder, &mut temp_writer)?;

        Ok(temp_file)
    }

    /// Loads samples from a SQLite database file
    fn load_samples_from_db(
        db_path: &Path,
    ) -> Result<(Vec<ReversiSample>, Vec<ReversiSample>), BoxError> {
        // Create SQLite datasets for each split
        let train_dataset = SqliteDataset::<ReversiSample>::from_db_file(db_path, "train")?;
        let valid_dataset = SqliteDataset::<ReversiSample>::from_db_file(db_path, "valid")?;

        // Collect all samples from each dataset
        let train_samples = (0..train_dataset.len())
            .filter_map(|i| train_dataset.get(i))
            .collect::<Vec<_>>();

        let valid_samples = (0..valid_dataset.len())
            .filter_map(|i| valid_dataset.get(i))
            .collect::<Vec<_>>();

        Ok((train_samples, valid_samples))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dataset_loader() {
        // This test would require an actual compressed dataset file
        // For now, we'll skip the implementation
        // let loader = DatasetLoader::load_from_compressed("work/dataset", "records").unwrap();
        // assert!(loader.train_dataset.len() > 0);
        // assert!(loader.valid_dataset.len() > 0);
    }
}
