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
    /// * `records_path` - Full path to the compressed dataset file (with or without .gz extension)
    ///
    /// # Returns
    ///
    /// A DatasetLoader containing both training and validation datasets
    pub fn load_from_compressed(records_path: &str) -> Result<Self, BoxError> {
        let dataset_path = if records_path.ends_with(".gz") {
            records_path.to_string()
        } else {
            format!("{}.gz", records_path)
        };

        // Decompress the .gz file to a temporary file
        let temp_db = Self::decompress_dataset(&dataset_path)?;

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
    fn decompress_dataset(dataset_path: &str) -> Result<NamedTempFile, BoxError> {
        let gz_file = File::open(dataset_path)?;
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
    use crate::{
        dataset_generator::{DatasetGeneratorConfig, EvaluatorType, StrategyType},
        test_utils::{MockProgressReporter, TestCleanup},
    };
    use std::fs;

    #[test]
    fn test_load_generated_dataset_integration() {
        let temp_dir = std::env::temp_dir().join("reversi_loader_test");
        let _ = fs::create_dir_all(&temp_dir);

        // Setup cleanup
        let mut cleanup = TestCleanup::new();
        cleanup.add_file(temp_dir.join("test_dataset.gz"));
        cleanup.add_file(temp_dir.join("test_dataset"));
        cleanup.add_dir(&temp_dir);

        // Generate a small dataset
        let config = DatasetGeneratorConfig {
            train_records: 2,
            valid_records: 1,
            num_random_moves: 2,
            search_depth: 1,
            evaluator: EvaluatorType::PhaseAware,
            order_evaluator: EvaluatorType::PhaseAware,
            strategy: StrategyType::NegaScount,
            output_dir: temp_dir.to_string_lossy().to_string(),
            output_name: "test_dataset".to_string(),
        };

        let generator = config.init();
        let progress = MockProgressReporter;

        // Generate dataset
        let gen_result = generator.generate_dataset(&progress);
        assert!(gen_result.is_ok(), "Dataset generation should succeed");

        // Verify compressed file exists
        let gz_file = temp_dir.join("test_dataset.gz");
        assert!(gz_file.exists(), "Compressed dataset should exist");

        // Load the dataset
        let gz_path = temp_dir.join("test_dataset.gz");
        let loader_result = DatasetLoader::load_from_compressed(&gz_path.to_string_lossy());

        assert!(loader_result.is_ok(), "Dataset loading should succeed");

        let loader = loader_result.unwrap();

        // Verify datasets are not empty
        assert!(
            loader.train_dataset.len() > 0,
            "Training dataset should not be empty"
        );
        assert!(
            loader.valid_dataset.len() > 0,
            "Validation dataset should not be empty"
        );

        // Verify we can access samples
        let train_sample = loader.train_dataset.get(0);
        assert!(
            train_sample.is_some(),
            "Should be able to get training sample"
        );

        let valid_sample = loader.valid_dataset.get(0);
        assert!(
            valid_sample.is_some(),
            "Should be able to get validation sample"
        );

        // Verify sample structure
        if let Some(sample) = train_sample {
            // ReversiSample has black_bits, white_bits, and stone_diff fields
            assert!(
                sample.black_bits != 0 || sample.white_bits != 0,
                "Sample should have some stones on the board"
            );
            assert!(
                sample.stone_diff >= -64 && sample.stone_diff <= 64,
                "Stone difference should be within valid range"
            );
        }

        // Cleanup happens automatically
    }
}
