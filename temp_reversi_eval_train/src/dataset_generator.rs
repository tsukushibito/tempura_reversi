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

#[derive(Config)]
pub enum EvaluatorType {
    PhaseAware,
}

#[derive(Config)]
pub enum StrategyType {
    NegaScount,
}

#[derive(Config)]
pub struct DatasetGeneratorConfig {
    #[config(default = 100)]
    pub num_records: usize,

    #[config(default = 10)]
    pub num_random_moves: usize,

    #[config(default = 5)]
    pub search_depth: usize,

    #[config(default = "EvaluatorType::PhaseAware")]
    pub evaluator: EvaluatorType,

    #[config(default = "EvaluatorType::PhaseAware")]
    pub order_evaluator: EvaluatorType,

    #[config(default = "StrategyType::NegaScount")]
    pub strategy: StrategyType,

    #[config(default = "String::from(\"work/dataset\")")]
    pub output_dir: String,

    #[config(default = "String::from(\"records\")")]
    pub output_name: String,

    #[config(default = "String::from(\"train\")")]
    pub split_name: String,
}

impl DatasetGeneratorConfig {
    pub fn init(&self) -> DatasetGenerator {
        DatasetGenerator {
            config: self.clone(),
        }
    }
}

pub struct DatasetGenerator {
    config: DatasetGeneratorConfig,
}

pub trait ProgressReporter: Clone + Send + Sync {
    fn increment(&self, delta: u64);
    fn finish(&self);
    fn set_message(&self, message: &str);
}

impl DatasetGenerator {
    pub fn generate_dataset(&self, progress: &impl ProgressReporter) -> Result<(), BoxError> {
        let mut writer = self.open_writer()?;

        for (start, end) in self.batch_ranges() {
            let records = self.generate_batch(start, end, progress);
            self.write_batch(&writer, &records)?;
            progress.set_message(&format!("Batch {}-{} completed", start, end - 1));
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

    fn batch_ranges(&self) -> impl Iterator<Item = (usize, usize)> + use<'_> {
        (0..self.config.num_records)
            .step_by(Self::BATCH_SIZE)
            .map(move |start| {
                let end = (start + Self::BATCH_SIZE).min(self.config.num_records);
                (start, end)
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
    ) -> Result<(), BoxError> {
        for record in records {
            for sample in record.to_samples() {
                writer.write(&self.config.split_name, &sample)?;
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
