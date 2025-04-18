use indicatif::ProgressBar;
use temp_reversi_eval_train::game_record_generator::{
    EvaluatorType, GameRecordGeneratorConfig, ProgressReporter, StrategyType,
};

#[derive(Clone)]
struct CliProgressReporter {
    pb: ProgressBar,
}

impl CliProgressReporter {
    fn new(total: usize) -> Self {
        let pb = ProgressBar::new(total as u64);
        pb.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({percent}%) {msg}")
                .unwrap()
                .progress_chars("█▓▒░ ")
        );
        Self { pb }
    }
}

impl ProgressReporter for CliProgressReporter {
    fn increment(&self, delta: u64) {
        self.pb.inc(delta);
        if self.pb.position() % 100 == 0 {}
    }

    fn finish(&self) {
        self.pb.finish();
    }

    fn set_message(&self, message: &str) {
        self.pb.set_message(message.to_string());
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = GameRecordGeneratorConfig {
        num_records: 10000,
        num_random_moves: 10,
        search_depth: 5,
        evaluator: EvaluatorType::PhaseAware,
        order_evaluator: EvaluatorType::PhaseAware,
        strategy: StrategyType::NegaScount,
        output_dir: String::from("work/dataset"),
        output_name: String::from("records"),
        split_name: String::from("train"),
    };
    let generator = config.init();
    let progress = CliProgressReporter::new(config.num_records);
    generator.generate_records(&progress)?;
    Ok(())
}
