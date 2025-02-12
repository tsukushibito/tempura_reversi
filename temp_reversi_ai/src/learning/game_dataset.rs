use super::{extract_features, Dataset};
use crate::{evaluation::PatternEvaluator, patterns::get_predefined_patterns, utils::Feature};
use rand::seq::SliceRandom;
use rayon::prelude::*;
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

    /// Loads multiple dataset files (either a single bin file or multiple parts),
    /// merges them into one GameDataset, shuffles the records, and then splits them
    /// into training and validation datasets according to the given train_ratio.
    ///
    /// # Arguments
    ///
    /// * `base_file_name` - The base file name for the dataset files.
    /// * `train_ratio` - The ratio of records to be used for training (e.g., 0.8 for 80%).
    ///
    /// # Returns
    ///
    /// A tuple (training_dataset, validation_dataset) or an io::Error if no data is found.
    pub fn load_auto(base_file_name: &str, train_ratio: f32) -> std::io::Result<(Self, Self)> {
        let bin_file = format!("{}.bin", base_file_name);
        let combined_dataset = if metadata(&bin_file).is_ok() {
            Self::load_bin(&bin_file)?
        } else {
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
            combined_dataset
        };

        if combined_dataset.records.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No dataset files found",
            ));
        }

        let mut records = combined_dataset.records;
        records.shuffle(&mut rand::thread_rng());
        let split_index = ((records.len() as f32) * train_ratio).round() as usize;
        let (train_records, validation_records) = records.split_at(split_index);

        Ok((
            GameDataset {
                records: train_records.to_vec(),
            },
            GameDataset {
                records: validation_records.to_vec(),
            },
        ))
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
    ) -> impl Iterator<Item = Dataset> + '_ {
        let evaluator = PatternEvaluator::new(get_predefined_patterns());

        self.records
            .par_chunks(batch_size)
            .map(move |chunk| {
                let mut batch = Dataset::new();
                for record in chunk.iter() {
                    let final_score = (record.final_score.0 as f32) - (record.final_score.1 as f32);
                    let mut game = Game::default();
                    let mut phase = 0;
                    for &pos_idx in &record.moves {
                        let pos = Position::from_u8(pos_idx).unwrap();
                        if game.is_valid_move(pos) {
                            let feature_vector = extract_features(&game.board_state(), &evaluator);
                            let feature = Feature {
                                phase,
                                vector: feature_vector,
                            };
                            batch.add_sample(feature, final_score);
                            game.apply_move(pos).unwrap();
                            phase += 1;
                        }
                    }
                }
                batch
            })
            // Collect the parallel iterator into a Vec and convert it back to an iterator.
            .collect::<Vec<Dataset>>()
            .into_iter()
    }

    /// Shuffles the game records in the dataset.
    pub fn shuffle(&mut self) {
        self.records.shuffle(&mut rand::thread_rng());
    }
}
