use burn::{
    data::{dataloader::DataLoaderBuilder, dataset::Dataset},
    optim::AdamConfig,
    prelude::*,
    record::{CompactRecorder, NoStdTrainingRecorder},
    tensor::backend::AutodiffBackend,
    train::{metric::LossMetric, LearnerBuilder},
};

use crate::{
    dataset::ReversiBatcher, dataset_loader::DatasetLoader, model::ReversiModelConfig,
    visualizer::generate_loss_plot,
};

#[derive(Config)]
pub struct TrainingConfig {
    #[config(default = 8)]
    pub num_epochs: usize,

    #[config(default = 2)]
    pub num_workers: usize,

    #[config(default = 1337)]
    pub seed: u64,

    pub optimizer: AdamConfig,

    #[config(default = 15360)] // 256 * 60
    pub batch_size: usize,
}

fn create_artifact_dir(artifact_dir: &str) {
    // Remove existing artifacts before to get an accurate learner summary
    std::fs::remove_dir_all(artifact_dir).ok();
    std::fs::create_dir_all(artifact_dir).ok();
}

pub fn run<B: AutodiffBackend>(
    config: TrainingConfig,
    artifact_dir: &str,
    records_dir: &str,
    records_name: &str,
    device: B::Device,
) -> Result<(), Box<dyn std::error::Error>> {
    create_artifact_dir(artifact_dir);

    // Config
    let model = ReversiModelConfig::new().init(&device);
    B::seed(config.seed);

    // Load datasets from compressed SQLite file
    let loader = DatasetLoader::load_from_compressed(records_dir, records_name)?;
    let train_dataset = loader.train_dataset;
    let valid_dataset = loader.valid_dataset;

    println!("Train Dataset Size: {}", train_dataset.len());
    println!("Valid Dataset Size: {}", valid_dataset.len());

    let batcher_train = ReversiBatcher::<B>::new(device.clone());

    let batcher_test = ReversiBatcher::<B::InnerBackend>::new(device.clone());

    let dataloader_train = DataLoaderBuilder::new(batcher_train)
        .batch_size(config.batch_size)
        .shuffle(config.seed)
        .num_workers(config.num_workers)
        .build(train_dataset);

    let dataloader_test = DataLoaderBuilder::new(batcher_test)
        .batch_size(config.batch_size)
        .shuffle(config.seed)
        .num_workers(config.num_workers)
        .build(valid_dataset);

    // Model
    let learner = LearnerBuilder::new(artifact_dir)
        .metric_train_numeric(LossMetric::new())
        .metric_valid_numeric(LossMetric::new())
        .with_file_checkpointer(CompactRecorder::new())
        .devices(vec![device.clone()])
        .num_epochs(config.num_epochs)
        .summary()
        .build(model, config.optimizer.init(), 1e-3);

    let model_trained = learner.fit(dataloader_train, dataloader_test);

    config.save(format!("{artifact_dir}/config.json").as_str())?;

    model_trained.save_file(
        format!("{artifact_dir}/model"),
        &NoStdTrainingRecorder::new(),
    )?;

    println!("🎨 Generating loss plot...");
    match generate_loss_plot(artifact_dir) {
        Ok(()) => println!("✅ Loss plot generated successfully"),
        Err(e) => eprintln!("⚠️  Failed to generate loss plot: {}", e),
    }

    Ok(())
}
