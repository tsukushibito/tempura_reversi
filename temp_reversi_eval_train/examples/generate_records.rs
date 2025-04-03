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
        pb.set_style(indicatif::ProgressStyle::default_bar());
        Self { pb }
    }
}

impl ProgressReporter for CliProgressReporter {
    fn increment(&self, delta: u64) {
        self.pb.inc(delta);
    }

    fn finish(&self) {
        self.pb.finish();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = GameRecordGeneratorConfig {
        num_records: 10000,
        search_depth: 5,
        evaluator: EvaluatorType::PhaseAware,
        order_evaluator: EvaluatorType::PhaseAware,
        strategy: StrategyType::NegaScount,
        output_dir: String::from("work/dataset"),
        output_name: String::from("records"),
    };
    let generator = config.init();
    let progress = CliProgressReporter::new(config.num_records);
    generator.generate_records(&progress)?;
    Ok(())
}
