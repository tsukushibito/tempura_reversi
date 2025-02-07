use super::{extract_features, Dataset};
use crate::{
    evaluation::{EvaluationFunction, PatternEvaluator},
    patterns::get_predefined_patterns,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, metadata},
    path::Path,
};
use temp_reversi_core::{Game, Position};

/// Represents a game record containing move history and final score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameRecord {
    /// Sequence of moves represented as board indices (0-63).
    pub moves: Vec<u8>,
    /// Final score of the game, represented as (black, white).
    pub final_score: (u8, u8),
}

impl GameRecord {
    /// Creates a new `GameRecord` from a completed game.
    pub fn new(game: &Game) -> Self {
        let moves = game.history().iter().map(|m| m.to_u8()).collect();
        let (black_score, white_score) = game.current_score();

        Self {
            moves,
            final_score: (black_score as u8, white_score as u8),
        }
    }
}

/// Manages multiple `GameRecord` entries, supporting batch processing, saving, and loading.
#[derive(Debug, Serialize, Deserialize)]
pub struct GameDataset {
    /// A collection of game records.
    pub records: Vec<GameRecord>,
}

impl GameDataset {
    /// Creates a new, empty `GameDataset`.
    ///
    /// # Returns
    ///
    /// A new instance of `GameDataset` with no records.
    ///
    /// # Example
    ///
    /// ```
    /// let dataset = GameDataset::new();
    /// assert!(dataset.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    /// Adds a game record to the dataset.
    ///
    /// # Arguments
    ///
    /// * `record` - A `GameRecord` containing move history and final score.
    ///
    /// # Example
    ///
    /// ```
    /// let mut dataset = GameDataset::new();
    /// let record = GameRecord { moves: vec![0, 1, 2], final_score: (32, 32) };
    /// dataset.add_record(record);
    /// assert_eq!(dataset.len(), 1);
    /// ```
    pub fn add_record(&mut self, record: GameRecord) {
        self.records.push(record);
    }

    /// Returns the number of records in the dataset.
    ///
    /// # Returns
    ///
    /// The number of game records stored in the dataset.
    ///
    /// # Example
    ///
    /// ```
    /// let dataset = GameDataset::new();
    /// assert_eq!(dataset.len(), 0);
    /// ```
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Checks if the dataset is empty.
    ///
    /// # Returns
    ///
    /// * `true` if the dataset contains no records.
    /// * `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// let dataset = GameDataset::new();
    /// assert!(dataset.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Saves the dataset in binary format using bincode serialization.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The file path to save the dataset.
    ///
    /// # Returns
    ///
    /// A `std::io::Result<()>` indicating success or failure.
    ///
    /// # Example
    ///
    /// ```
    /// let dataset = GameDataset::new();
    /// dataset.save_bin("dataset.bin").unwrap();
    /// ```
    pub fn save_bin(&self, file_path: &str) -> std::io::Result<()> {
        let encoded: Vec<u8> = bincode::serialize(self).unwrap();
        fs::write(file_path, encoded)
    }

    /// Loads a dataset from a binary file.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The file path to load the dataset from.
    ///
    /// # Returns
    ///
    /// A `std::io::Result<GameDataset>` containing the loaded dataset or an error.
    ///
    /// # Example
    ///
    /// ```
    /// let dataset = GameDataset::load_bin("dataset.bin").unwrap();
    /// ```
    pub fn load_bin(file_path: &str) -> std::io::Result<Self> {
        let data = fs::read(file_path)?;
        let dataset: Self = bincode::deserialize(&data).unwrap();
        Ok(dataset)
    }

    /// Saves the dataset in chunks of 100,000 records to avoid large file sizes.
    ///
    /// # Arguments
    ///
    /// * `base_file_name` - The base name for the output files.
    ///
    /// # Returns
    ///
    /// A `std::io::Result<()>` indicating success or failure.
    pub fn save_auto(&self, base_file_name: &str) -> std::io::Result<()> {
        // If base_file_name includes a directory, create it if it doesn't exist.
        if let Some(parent) = Path::new(base_file_name).parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }

        const MAX_RECORDS_PER_FILE: usize = 100_000;

        if self.records.len() <= MAX_RECORDS_PER_FILE {
            return self.save_bin(&format!("{}.bin", base_file_name));
        }

        for (i, chunk) in self.records.chunks(MAX_RECORDS_PER_FILE).enumerate() {
            let part_dataset = GameDataset {
                records: chunk.to_vec(),
            };
            part_dataset.save_bin(&format!("{}_part_{}.bin", base_file_name, i + 1))?;
        }

        Ok(())
    }

    /// Loads multiple dataset files and merges them into a single dataset.
    ///
    /// # Arguments
    ///
    /// * `base_file_name` - The base name for the input files.
    ///
    /// # Returns
    ///
    /// A `std::io::Result<GameDataset>` containing the merged dataset or an error.
    pub fn load_auto(base_file_name: &str) -> std::io::Result<Self> {
        let bin_file = format!("{}.bin", base_file_name);
        if metadata(&bin_file).is_ok() {
            return Self::load_bin(&bin_file);
        }

        let mut combined_dataset = GameDataset::new();
        let mut part_num = 1;

        loop {
            let file_name = format!("{}_part_{}.bin", base_file_name, part_num);
            if let Ok(dataset) = Self::load_bin(&file_name) {
                combined_dataset.records.extend(dataset.records);
                part_num += 1;
            } else {
                break;
            }
        }

        if combined_dataset.records.is_empty() {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No dataset files found",
            ))
        } else {
            Ok(combined_dataset)
        }
    }

    /// Extracts training data in batches from the game records.
    ///
    /// # Arguments
    ///
    /// * `batch_size` - The number of records per batch.
    ///
    /// # Returns
    ///
    /// An iterator over `Dataset` batches.
    ///
    /// # Example
    ///
    /// ```
    /// let dataset = GameDataset::load_bin("dataset.bin").unwrap();
    /// let mut batches = dataset.extract_training_data_in_batches(100);
    /// let first_batch = batches.next().unwrap();
    /// assert!(!first_batch.is_empty());
    /// ```
    pub fn extract_training_data_in_batches(
        &self,
        batch_size: usize,
    ) -> impl Iterator<Item = Dataset> + use<'_> {
        let evaluator = PatternEvaluator::new(get_predefined_patterns());
        let mut batch = Dataset::new();

        self.records.chunks(batch_size).map(move |chunk| {
            batch.features.clear();
            batch.labels.clear();

            for record in chunk.iter().cloned() {
                let mut game = Game::default();
                for &pos_idx in &record.moves {
                    let pos = Position::from_u8(pos_idx).unwrap();
                    if game.is_valid_move(pos) {
                        let feature_vector = extract_features(&game.board_state());
                        let score = evaluator.evaluate(&game.board_state(), game.current_player());
                        batch.add_sample(feature_vector, score as f32);
                        game.apply_move(pos).unwrap();
                    }
                }
            }

            batch.clone()
        })
    }
}
