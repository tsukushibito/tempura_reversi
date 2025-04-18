use burn::{config::Config, data::dataset::SqliteDatasetStorage};
use rayon::prelude::*;
use temp_reversi_ai::{
    ai_player::AiPlayer,
    evaluator::PhaseAwareEvaluator,
    strategy::{NegaScoutStrategy, RandomStrategy},
};
use temp_reversi_core::{Game, GamePlayer};

use crate::{dataset::ReversiSample, game_record::GameRecord};

#[derive(Config)]
pub enum EvaluatorType {
    PhaseAware,
}

#[derive(Config)]
pub enum StrategyType {
    NegaScount,
}

#[derive(Config)]
pub struct GameRecordGeneratorConfig {
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

impl GameRecordGeneratorConfig {
    pub fn init(&self) -> GameRecordGenerator {
        GameRecordGenerator {
            config: self.clone(),
        }
    }
}

pub struct GameRecordGenerator {
    config: GameRecordGeneratorConfig,
}

pub trait ProgressReporter: Clone + Send + Sync {
    fn increment(&self, delta: u64);
    fn finish(&self);
    fn set_message(&self, message: &str);
}

impl GameRecordGenerator {
    pub fn generate_records(
        &self,
        progress: &impl ProgressReporter,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let storage = SqliteDatasetStorage::from_file(self.config.output_name.clone())
            .with_base_dir(self.config.output_dir.clone());

        let mut writer = storage.writer::<ReversiSample>(true)?;

        const BATCH_SIZE: usize = 1000;

        for batch_start in (0..self.config.num_records).step_by(BATCH_SIZE) {
            let batch_end = (batch_start + BATCH_SIZE).min(self.config.num_records);
            let batch_size = batch_end - batch_start;

            let batch_records: Vec<GameRecord> = (0..batch_size)
                .into_par_iter()
                .map_with(progress.clone(), |p, _| {
                    let evaluator = match self.config.evaluator {
                        EvaluatorType::PhaseAware => PhaseAwareEvaluator::default(),
                    };
                    let order_evaluator = match self.config.order_evaluator {
                        EvaluatorType::PhaseAware => PhaseAwareEvaluator::default(),
                    };
                    let strategy = match self.config.strategy {
                        StrategyType::NegaScount => NegaScoutStrategy::new(
                            evaluator,
                            order_evaluator,
                            self.config.search_depth,
                        ),
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

                    p.increment(1);

                    GameRecord { moves, final_score }
                })
                .collect();

            for record in &batch_records {
                let samples = record.to_samples();
                for sample in samples {
                    writer.write(&self.config.split_name, &sample)?;
                }
            }

            progress.set_message(&format!(
                "Batch {}-{} completed and saved to SQLite",
                batch_start,
                batch_end - 1
            ));
        }

        writer.set_completed()?;

        progress.finish();
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
        let dir = "test_game_record_generator/dataset";
        let _ = std::fs::remove_dir_all(dir); // Clean up before test

        let config = GameRecordGeneratorConfig {
            num_records: 10,
            num_random_moves: 10,
            search_depth: 5,
            evaluator: EvaluatorType::PhaseAware,
            order_evaluator: EvaluatorType::PhaseAware,
            strategy: StrategyType::NegaScount,
            output_dir: String::from(dir),
            output_name: String::from("records"),
            split_name: String::from("train"),
        };
        let generator = config.init();
        let progress = MockProgressReporter {};
        generator.generate_records(&progress).unwrap();

        let records = GameRecord::load_records(dir, "records").unwrap();
        assert!(!records.is_empty(), "Records should not be empty");

        assert_eq!(records.len(), 10, "Should generate 10 records");

        let _ = std::fs::remove_dir_all(dir); // Clean up after test
    }
}
