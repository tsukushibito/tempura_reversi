use burn::{
    data::{dataloader::DataLoaderBuilder, dataset::Dataset},
    optim::AdamConfig,
    prelude::*,
    record::{CompactRecorder, NoStdTrainingRecorder},
    tensor::backend::AutodiffBackend,
    train::{metric::LossMetric, LearnerBuilder},
};
use temp_reversi_eval::{feature::PHASE_COUNT, runtime_model::RuntimeModel};

use crate::{
    dataset::ReversiBatcher,
    dataset_loader::DatasetLoader,
    feature_packer::FEATURE_PACKER,
    model::{ReversiModel, ReversiModelConfig},
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

/// Extracts weights from ReversiModel and converts to RuntimeModel format
fn extract_runtime_model<B: Backend>(model: &ReversiModel<B>) -> RuntimeModel {
    // Get the embedding weights tensor
    let weights_tensor = model.feature_weights.weight.val();

    // Convert tensor to Vec<f32>
    let weights_flat: Vec<f32> = weights_tensor.into_data().into_vec().unwrap();

    // Reshape into phase-based structure
    let num_features = FEATURE_PACKER.packed_feature_size;
    let mut weights: Vec<Vec<f32>> = Vec::with_capacity(PHASE_COUNT as usize);

    for phase in 0..PHASE_COUNT as usize {
        let start_idx = phase * num_features;
        let end_idx = start_idx + num_features;
        weights.push(weights_flat[start_idx..end_idx].to_vec());
    }

    RuntimeModel { weights }
}

pub fn run<B: AutodiffBackend>(
    config: TrainingConfig,
    artifact_dir: &str,
    records_path: &str,
    runtime_model_path: &str,
    device: B::Device,
) -> Result<(), Box<dyn std::error::Error>> {
    create_artifact_dir(artifact_dir);

    // Config
    let model = ReversiModelConfig::new().init(&device);
    B::seed(config.seed);

    // Load datasets from compressed SQLite file
    let loader = DatasetLoader::load_from_compressed(records_path)?;
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
    let runtime_model = extract_runtime_model(&model_trained);

    config.save(format!("{artifact_dir}/config.json").as_str())?;

    model_trained.save_file(
        format!("{artifact_dir}/model"),
        &NoStdTrainingRecorder::new(),
    )?;

    // runtime_model.save(runtime_model_path)?;
    runtime_model.save_uncompressed(runtime_model_path)?;

    println!("🎨 Generating loss plot...");
    match generate_loss_plot(artifact_dir) {
        Ok(()) => println!("✅ Loss plot generated successfully"),
        Err(e) => eprintln!("⚠️  Failed to generate loss plot: {}", e),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn::backend::NdArray;

    type TestBackend = NdArray;

    #[test]
    fn test_extract_runtime_model() {
        // Create a test device
        let device = Default::default();

        // Initialize a model
        let model = ReversiModelConfig::new().init::<TestBackend>(&device);

        // Extract runtime model
        let runtime_model = extract_runtime_model(&model);

        // Verify structure
        assert_eq!(runtime_model.weights.len(), PHASE_COUNT as usize);

        for phase_weights in &runtime_model.weights {
            assert_eq!(phase_weights.len(), FEATURE_PACKER.packed_feature_size);
        }

        // Verify total number of weights matches model
        let total_weights: usize = runtime_model.weights.iter().map(|w| w.len()).sum();
        let expected_total = PHASE_COUNT as usize * FEATURE_PACKER.packed_feature_size;
        assert_eq!(total_weights, expected_total);
    }

    #[test]
    fn test_extract_runtime_model_weights_consistency() {
        let device = Default::default();
        let model = ReversiModelConfig::new().init::<TestBackend>(&device);

        // Get original weights
        let original_weights = model.feature_weights.weight.val();
        let original_flat: Vec<f32> = original_weights.into_data().into_vec().unwrap();

        // Extract runtime model
        let runtime_model = extract_runtime_model(&model);

        // Flatten runtime model weights
        let runtime_flat: Vec<f32> = runtime_model.weights.into_iter().flatten().collect();

        // Verify weights are identical
        assert_eq!(original_flat.len(), runtime_flat.len());
        for (original, runtime) in original_flat.iter().zip(runtime_flat.iter()) {
            assert!(
                (original - runtime).abs() < 1e-6,
                "Weight mismatch: original={}, runtime={}",
                original,
                runtime
            );
        }
    }

    #[test]
    fn test_extract_runtime_model_phase_structure() {
        let device = Default::default();
        let model = ReversiModelConfig::new().init::<TestBackend>(&device);

        let runtime_model = extract_runtime_model(&model);

        // Verify each phase has correct number of features
        for (phase_idx, phase_weights) in runtime_model.weights.iter().enumerate() {
            assert_eq!(
                phase_weights.len(),
                FEATURE_PACKER.packed_feature_size,
                "Phase {} has incorrect number of features",
                phase_idx
            );
        }

        // Verify we have the right number of phases
        assert_eq!(
            runtime_model.weights.len(),
            PHASE_COUNT as usize,
            "Incorrect number of phases"
        );
    }
}
