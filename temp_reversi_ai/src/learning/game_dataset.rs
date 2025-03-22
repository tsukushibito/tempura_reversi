use super::{extract_features, Dataset};
use crate::{
    patterns::{get_predefined_patterns, PatternGroup},
    utils::Feature,
};
use lz4_flex::{compress_prepend_size, decompress_size_prepended};
use rand::seq::SliceRandom;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, metadata},
    path::Path,
};
use temp_reversi_core::{Bitboard, Game, Position};

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

    /// Saves the dataset in binary format using bincode serialization with LZ4 compression.
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
        let compressed = compress_prepend_size(&encoded);
        fs::write(file_path, compressed)
    }

    /// Loads a dataset from a binary file using LZ4 decompression.
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
        let decompressed = decompress_size_prepended(&data).expect("Failed to decompress dataset.");
        let dataset: Self = bincode::deserialize(&decompressed).unwrap();
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

    pub fn load_auto(base_file_name: &str) -> std::io::Result<Self> {
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
        records.shuffle(&mut rand::rng());

        Ok(GameDataset { records })
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
        let groups = get_predefined_patterns();
        self.records.chunks(batch_size).map(move |chunk| {
            let samples: Vec<(Feature, f32)> = chunk
                .par_iter()
                .flat_map(|record| Self::process_record(record, &groups))
                .collect();
            let mut batch = Dataset::new();
            for (feature, label) in samples {
                batch.add_sample(feature, label);
            }
            batch
        })
    }

    /// Extracts training data from all game records into a single Dataset.
    /// The processing is parallelized where applicable.
    pub fn extract_all_training_data(&self) -> Dataset {
        let groups = get_predefined_patterns();
        let samples: Vec<(Feature, f32)> = self
            .records
            .par_iter()
            .flat_map_iter(|record| Self::process_record(record, &groups))
            .collect();

        let mut dataset = Dataset::new();
        for (feature, label) in samples {
            dataset.add_sample(feature, label);
        }
        dataset
    }

    /// Shuffles the game records in the dataset.
    pub fn shuffle(&mut self) {
        self.records.shuffle(&mut rand::rng());
    }

    pub fn process_record(record: &GameRecord, groups: &[PatternGroup]) -> Vec<(Feature, f32)> {
        let final_score = (record.final_score.0 as f32) - (record.final_score.1 as f32);
        let mut samples = Vec::new();
        let mut game = Game::default();
        for &pos_idx in &record.moves {
            let pos = Position::from_u8(pos_idx);
            if game.is_valid_move(pos) {
                game.apply_move(pos).unwrap();
                let board: &Bitboard = game.board_state();
                let feature_vector = extract_features(board, groups);
                let (b, w) = board.count_stones();
                let phase = 65 - b - w;
                samples.push((
                    Feature {
                        phase,
                        vector: feature_vector,
                    },
                    final_score,
                ));

                // Add the inverted board state as well
                let inverted_board = Bitboard::new(board.bits().1, board.bits().0);
                let feature_vector = extract_features(&inverted_board, groups);
                samples.push((
                    Feature {
                        phase,
                        vector: feature_vector,
                    },
                    -final_score,
                ));
            } else {
                break;
            }
        }
        samples
    }
}
