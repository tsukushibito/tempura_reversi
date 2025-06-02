use burn::{
    backend::{ndarray::NdArrayDevice, Autodiff, NdArray},
    optim::AdamConfig,
};
use indicatif::ProgressBar;
use std::path::Path;
use temp_reversi_eval_train::{
    dataset_generator::{DatasetGeneratorConfig, EvaluatorType, ProgressReporter, StrategyType},
    training,
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

fn ensure_dataset_exists() -> Result<(), Box<dyn std::error::Error>> {
    let dataset_path = Path::new("work/datasets/dataset.gz");

    if dataset_path.exists() {
        println!("✅ Dataset already exists at: {}", dataset_path.display());
        return Ok(());
    }

    println!("📊 Dataset not found. Generating dataset...");

    // Create directory if it doesn't exist
    if let Some(parent) = dataset_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let config = DatasetGeneratorConfig {
        train_records: 8000,
        valid_records: 2000,
        num_random_moves: 10,
        search_depth: 5,
        evaluator: EvaluatorType::PhaseAware,
        order_evaluator: EvaluatorType::PhaseAware,
        strategy: StrategyType::NegaScount,
        output_dir: String::from("work/datasets"),
        output_name: String::from("dataset"),
    };

    let generator = config.init();
    let progress = CliProgressReporter::new(config.train_records + config.valid_records);
    generator.generate_dataset(&progress)?;

    println!("✅ Dataset generation completed!");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 Reversi Model Training");
    println!("========================");

    // Ensure dataset exists
    ensure_dataset_exists()?;

    // Start training
    println!("🚀 Starting model training...");
    let device = NdArrayDevice::Cpu;
    // let device = WgpuDevice::DefaultDevice;

    let config = training::TrainingConfig {
        num_epochs: 16,
        num_workers: 8,
        seed: 1337,
        optimizer: AdamConfig::new(),
        batch_size: 15360, // 256 * 60
    };

    training::run::<Autodiff<NdArray>>(
        // training::run::<Autodiff<Wgpu>>(
        config,
        "work/artifacts",
        "work/datasets",
        "dataset",
        device,
    )?;

    println!("✅ Training completed successfully!");
    println!("📁 Artifacts saved to: work/artifacts/");
    println!("📄 Model saved to: work/artifacts/model");
    println!("⚙️  Config saved to: work/artifacts/config.json");

    Ok(())
}
