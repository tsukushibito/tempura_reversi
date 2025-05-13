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
        let storage = SqliteDatasetStorage::from_file(self.config.output_name.clone())
            .with_base_dir(self.config.output_dir.clone());
        let writer = storage.writer::<ReversiSample>(true)?;
        Ok(writer)
    }

    fn compress_output(&self) -> Result<(), BoxError> {
        let db_path = Path::new(&self.config.output_dir).join(&self.config.output_name);
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

    #[derive(Clone)]
    struct MockProgressReporter {}

    impl ProgressReporter for MockProgressReporter {
        fn increment(&self, _: u64) {
            // Mock implementation
        }

        fn finish(&self) {
            // Mock implementation
        }

        fn set_message(&self, _message: &str) {
            // Mock implementation
        }
    }

    #[test]
    fn test_game_record_generator() {
        todo!()
    }
}
