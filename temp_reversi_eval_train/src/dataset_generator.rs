use std::{
    fs::{remove_file, File},
    io::copy,
    path::Path,
};

use burn::{
    config::Config,
    data::dataset::{SqliteDatasetStorage, SqliteDatasetWriter},
};
use flate2::{write::GzEncoder, Compression};
use rayon::prelude::*;
use temp_reversi_ai::{
    ai_player::AiPlayer,
    evaluator::PhaseAwareEvaluator,
    strategy::{NegaScoutStrategy, RandomStrategy},
};
use temp_reversi_core::{Game, GamePlayer};

use crate::{dataset::ReversiSample, game_record::GameRecord};

type BoxError = Box<dyn std::error::Error>;

/// Types of evaluators that can be used for position assessment
#[derive(Config)]
pub enum EvaluatorType {
    /// Phase-aware evaluator that adjusts evaluation based on game phase
    PhaseAware,
}

/// Types of search strategies that can be used for move selection
#[derive(Config)]
pub enum StrategyType {
    /// NegaScout search algorithm for move selection
    NegaScount,
}

/// Configuration for the dataset generation process
#[derive(Config)]
pub struct DatasetGeneratorConfig {
    /// Number of game records to generate for training
    #[config(default = 100)]
    pub train_records: usize,

    /// Number of game records to generate for validation
    #[config(default = 20)]
    pub valid_records: usize,

    /// Number of random moves to play at the beginning of each game
    /// to ensure position diversity
    #[config(default = 10)]
    pub num_random_moves: usize,

    /// Maximum search depth for the AI player's search algorithm
    #[config(default = 5)]
    pub search_depth: usize,

    /// Evaluator to use for position assessment
    #[config(default = "EvaluatorType::PhaseAware")]
    pub evaluator: EvaluatorType,

    /// Evaluator to use for move ordering in search
    #[config(default = "EvaluatorType::PhaseAware")]
    pub order_evaluator: EvaluatorType,

    /// Search strategy to use for move selection
    #[config(default = "StrategyType::NegaScount")]
    pub strategy: StrategyType,

    /// Directory to store the generated dataset
    #[config(default = "String::from(\"work/dataset\")")]
    pub output_dir: String,

    /// Base filename for the generated dataset (without extension)
    #[config(default = "String::from(\"records\")")]
    pub output_name: String,
}

impl DatasetGeneratorConfig {
    /// Initializes a new DatasetGenerator instance from this configuration
    pub fn init(&self) -> DatasetGenerator {
        DatasetGenerator {
            config: self.clone(),
        }
    }
}

pub struct DatasetGenerator {
    config: DatasetGeneratorConfig,
}

/// Progress reporting interface for tracking dataset generation progress
pub trait ProgressReporter: Clone + Send + Sync {
    /// Increments the progress counter by the specified amount
    fn increment(&self, delta: u64);

    /// Signals that the operation is complete
    fn finish(&self);

    /// Sets a status message for the current operation
    fn set_message(&self, message: &str);
}

impl DatasetGenerator {
    /// Generates a dataset consisting of training and validation data
    ///
    /// This function generates game records for both training and validation splits
    /// according to the configuration. The data is stored in a SQLite database and
    /// compressed as a .gz file.
    ///
    /// # Arguments
    ///
    /// * `progress` - A progress reporter implementation to track generation progress
    ///
    /// # Returns
    ///
    /// A result that is Ok if the dataset was successfully generated
    pub fn generate_dataset(&self, progress: &impl ProgressReporter) -> Result<(), BoxError> {
        let mut writer = self.open_writer()?;

        progress.set_message("Generating training data...");
        for (start, end) in self.batch_ranges(0, self.config.train_records) {
            let records = self.generate_batch(start, end, progress);
            self.write_batch(&writer, &records, "train")?;
            progress.set_message(&format!("Training batch {}-{} completed", start, end - 1));
        }

        progress.set_message("Generating validation data...");
        for (start, end) in self.batch_ranges(0, self.config.valid_records) {
            let records = self.generate_batch(start, end, progress);
            self.write_batch(&writer, &records, "valid")?;
            progress.set_message(&format!("Validation batch {}-{} completed", start, end - 1));
        }

        writer.set_completed()?;

        progress.set_message("Compressing output...");
        self.compress_output()?;

        progress.finish();

        Ok(())
    }

    const BATCH_SIZE: usize = 1000;

    fn open_writer(&self) -> Result<SqliteDatasetWriter<ReversiSample>, BoxError> {
        let output_dir = Path::new(&self.config.output_dir);
        let db_file_path = output_dir.join(&self.config.output_name);
        let storage = SqliteDatasetStorage::from_file(db_file_path);
        let writer = storage.writer::<ReversiSample>(true)?;
        Ok(writer)
    }

    fn compress_output(&self) -> Result<(), BoxError> {
        let db_path = Path::new(&self.config.output_dir).join(&self.config.output_name);

        if !db_path.exists() {
            let gz_path = db_path.with_extension("gz");
            let outpu = File::create(&gz_path)?;
            let encoder = GzEncoder::new(outpu, Compression::default());
            encoder.finish()?;
            return Ok(());
        }

        let gz_path = db_path.with_extension("gz");
        let mut input = File::open(&db_path)?;
        let output = File::create(&gz_path)?;
        let mut encoder = GzEncoder::new(output, Compression::default());
        copy(&mut input, &mut encoder)?;
        encoder.finish()?;

        remove_file(db_path)?;

        Ok(())
    }

    fn batch_ranges(
        &self,
        start_offset: usize,
        count: usize,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..count).step_by(Self::BATCH_SIZE).map(move |start| {
            let batch_start = start_offset + start;
            let batch_end = (batch_start + Self::BATCH_SIZE).min(start_offset + count);
            (batch_start, batch_end)
        })
    }

    fn generate_batch(
        &self,
        _start: usize,
        end: usize,
        progress: &impl ProgressReporter,
    ) -> Vec<GameRecord> {
        let batch_size = end - _start;
        (0..batch_size)
            .into_par_iter()
            .map_with(progress.clone(), |p, _| {
                p.increment(1);
                self.play_game()
            })
            .collect()
    }

    fn play_game(&self) -> GameRecord {
        let evaluator = match self.config.evaluator {
            EvaluatorType::PhaseAware => PhaseAwareEvaluator::default(),
        };
        let order_evaluator = match self.config.order_evaluator {
            EvaluatorType::PhaseAware => PhaseAwareEvaluator::default(),
        };
        let strategy = match self.config.strategy {
            StrategyType::NegaScount => {
                NegaScoutStrategy::new(evaluator, order_evaluator, self.config.search_depth)
            }
        };
        let mut player = AiPlayer::new(Box::new(strategy));

        let randam_strategy = RandomStrategy;
        let mut random_player = AiPlayer::new(Box::new(randam_strategy));

        let mut game = Game::default();
        let mut moves = Vec::new();
        while !game.is_over() {
            let mv = if moves.len() < self.config.num_random_moves {
                random_player.select_move(&game)
            } else {
                player.select_move(&game)
            };
            moves.push(mv.to_u8());
            let _ = game.apply_move(mv);
        }
        let final_score = game.current_score();
        let final_score = (final_score.0 as u8, final_score.1 as u8);

        GameRecord { moves, final_score }
    }

    fn write_batch(
        &self,
        writer: &SqliteDatasetWriter<ReversiSample>,
        records: &[GameRecord],
        split_name: &str,
    ) -> Result<(), BoxError> {
        for record in records {
            for sample in record.to_samples() {
                writer.write(split_name, &sample)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    use crate::test_utils::{MockProgressReporter, TestCleanup};

    #[test]
    fn test_generate_dataset_success() {
        let temp_dir = std::env::temp_dir().join("reversi_test");
        let _ = fs::create_dir_all(&temp_dir);

        // Setup cleanup - automatically handles cleanup on drop
        let mut cleanup = TestCleanup::new();
        cleanup.add_file(temp_dir.join("test_dataset.gz"));
        cleanup.add_file(temp_dir.join("test_dataset"));
        cleanup.add_dir(&temp_dir);

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

        // Execute dataset generation
        let result = generator.generate_dataset(&progress);

        // Verify successful completion
        assert!(result.is_ok(), "Dataset generation should succeed");

        // Verify output file exists (compressed)
        let expected_file = temp_dir.join("test_dataset.gz");
        assert!(
            expected_file.exists(),
            "Compressed output file should exist"
        );

        // Verify original database file is removed
        let db_file = temp_dir.join("test_dataset");
        assert!(
            !db_file.exists(),
            "Original database file should be removed after compression"
        );

        // Cleanup happens automatically when `cleanup` goes out of scope
    }

    #[test]
    fn test_generate_dataset_empty_records() {
        let temp_dir = std::env::temp_dir().join("reversi_test_empty");
        let _ = fs::create_dir_all(&temp_dir);

        // Setup cleanup
        let mut cleanup = TestCleanup::new();
        cleanup.add_file(temp_dir.join("empty_dataset.gz"));
        cleanup.add_file(temp_dir.join("empty_dataset"));
        cleanup.add_dir(&temp_dir);

        let config = DatasetGeneratorConfig {
            train_records: 0,
            valid_records: 0,
            num_random_moves: 2,
            search_depth: 1,
            evaluator: EvaluatorType::PhaseAware,
            order_evaluator: EvaluatorType::PhaseAware,
            strategy: StrategyType::NegaScount,
            output_dir: temp_dir.to_string_lossy().to_string(),
            output_name: "empty_dataset".to_string(),
        };

        let generator = config.init();
        let progress = MockProgressReporter;

        let result = generator.generate_dataset(&progress);

        // Should still succeed even with 0 records
        assert!(result.is_ok(), "Should handle empty dataset generation");

        // Verify that output file was created (even if empty)
        let output_file = temp_dir.join("empty_dataset.gz");
        assert!(
            output_file.exists(),
            "Empty compressed file should still be created"
        );

        // Cleanup happens automatically
    }

    #[test]
    fn test_batch_ranges() {
        let config = DatasetGeneratorConfig {
            train_records: 5,
            valid_records: 2,
            num_random_moves: 3,
            search_depth: 2,
            evaluator: EvaluatorType::PhaseAware,
            order_evaluator: EvaluatorType::PhaseAware,
            strategy: StrategyType::NegaScount,
            output_dir: "test_output".to_string(),
            output_name: "test_records".to_string(),
        };

        let generator = config.init();

        // Test batch ranges calculation
        let ranges: Vec<_> = generator.batch_ranges(0, 2500).collect();
        assert!(!ranges.is_empty(), "Should generate batch ranges");
        assert_eq!(ranges[0], (0, 1000), "First batch should be 0-1000");
        assert_eq!(ranges[1], (1000, 2000), "Second batch should be 1000-2000");
        assert_eq!(
            ranges[2],
            (2000, 2500),
            "Last batch should handle remainder"
        );

        // Test small batch
        let small_ranges: Vec<_> = generator.batch_ranges(10, 50).collect();
        assert_eq!(small_ranges.len(), 1, "Small batch should be single range");
        assert_eq!(small_ranges[0], (10, 60), "Should handle offset correctly");
    }

    #[test]
    fn test_game_record_generator() {
        let config = DatasetGeneratorConfig {
            train_records: 5,
            valid_records: 2,
            num_random_moves: 3,
            search_depth: 2,
            evaluator: EvaluatorType::PhaseAware,
            order_evaluator: EvaluatorType::PhaseAware,
            strategy: StrategyType::NegaScount,
            output_dir: "test_output".to_string(),
            output_name: "test_records".to_string(),
        };

        let generator = config.init();
        let progress = MockProgressReporter;

        // Test individual game generation
        let game_record = generator.play_game();

        // Verify game record structure
        assert!(!game_record.moves.is_empty(), "Game should have moves");
        assert!(
            game_record.moves.len() >= config.num_random_moves,
            "Game should have at least the minimum random moves"
        );

        // Verify final score is valid (sum should be <= 64 for reversi)
        let total_pieces = game_record.final_score.0 as usize + game_record.final_score.1 as usize;
        assert!(total_pieces <= 64, "Total pieces should not exceed 64");
        assert!(total_pieces > 0, "Game should have some pieces");

        // Test batch generation
        let batch_records = generator.generate_batch(0, 3, &progress);
        assert_eq!(
            batch_records.len(),
            3,
            "Batch should contain exactly 3 records"
        );

        // Verify all records in batch are valid
        for record in &batch_records {
            assert!(!record.moves.is_empty(), "Each record should have moves");
            let total = record.final_score.0 as usize + record.final_score.1 as usize;
            assert!(
                total <= 64 && total > 0,
                "Each record should have valid final score"
            );
        }
    }
}
