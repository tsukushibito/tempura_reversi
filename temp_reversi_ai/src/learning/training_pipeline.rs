use std::collections::HashMap;
use std::sync::Arc;

use crate::evaluation::{PatternEvaluator, PhaseAwareEvaluator};
use crate::learning::loss_function::MSELoss;
use crate::learning::optimizer::Adam;
use crate::learning::{extract_features, generate_self_play_data, GameDataset, Trainer};
use crate::patterns::get_predefined_patterns;
use crate::plotter::{plot_overall_loss, plot_phase_losses};
use crate::strategy::negamax::NegamaxStrategy;
use crate::utils::ProgressReporter;

use super::Model;

/// Configuration for the training pipeline.
pub struct TrainingConfig {
    /// Number of self-play games to generate.
    pub num_games: usize,
    /// Batch size for training.
    pub batch_size: usize,
    /// Number of epochs for model training.
    pub num_epochs: usize,
    /// Path to save the trained model.
    pub model_path: String,
    /// Path to save the generated game dataset.
    pub dataset_base_path: String,
    /// Ratio of training data to use for training.
    pub train_ratio: f32,
    /// Path for overall loss plot.
    pub overall_loss_plot_path: String,
    /// Path for phase loss plot.
    pub phase_loss_plot_path: String,
}

/// Training pipeline for self-play data generation and model training.
pub struct TrainingPipeline {
    config: TrainingConfig,
}

impl TrainingPipeline {
    /// Creates a new instance of the training pipeline.
    pub fn new(config: TrainingConfig) -> Self {
        Self { config }
    }

    /// Executes the full training pipeline: generates self-play data and trains the model.
    pub fn run(&self) {
        self.generate_self_play_data(None);
        self.train();
    }

    /// Generates self-play data using AI strategies and saves it to a file.
    pub fn generate_self_play_data(
        &self,
        reporter: Option<Arc<dyn ProgressReporter + Send + Sync>>,
    ) {
        let dataset = generate_self_play_data(
            self.config.num_games,
            Box::new(NegamaxStrategy::new(PhaseAwareEvaluator, 5)),
            Box::new(NegamaxStrategy::new(PhaseAwareEvaluator, 5)),
            reporter,
        );

        dataset
            .save_auto(&self.config.dataset_base_path)
            .expect("Failed to save self-play data.");
    }

    /// Loads the dataset and trains the model.
    pub fn train(&self) {
        // GameDataset::load_auto は (training_dataset, validation_dataset) のタプルを返す前提
        let (mut train_dataset, validation_dataset) = self.load_dataset();
        if train_dataset.records.is_empty() {
            panic!("Training dataset is empty, cannot determine feature size.");
        }

        // Generate a dummy board state to extract feature vector and determine its size.
        let dummy_board = temp_reversi_core::Bitboard::default();
        let evaluator = PatternEvaluator::new(get_predefined_patterns());
        let feature_vector = extract_features(&dummy_board, &evaluator);
        let feature_size = feature_vector.size();
        let learning_rate = 0.001;

        let optimizer = Adam::new(feature_size, learning_rate);
        let mut trainer = Trainer::new(
            feature_size,
            MSELoss,
            optimizer,
            self.config.batch_size,
            self.config.num_epochs,
        );

        trainer.train(&mut train_dataset, &validation_dataset);

        // Plot overall loss
        if let Err(e) = plot_overall_loss(
            &trainer.validation_overall_losses,
            &self.config.overall_loss_plot_path,
        ) {
            eprintln!("Failed to plot overall loss: {}", e);
        }

        // Convert phase loss data: Vec<Vec<(usize, f32)>> -> Vec<HashMap<usize, f32>>
        let phase_loss_data: Vec<HashMap<usize, f32>> = trainer
            .validation_phase_losses
            .iter()
            .map(|vec_phase| {
                let mut hm = HashMap::new();
                for &(phase, loss) in vec_phase {
                    hm.insert(phase, loss);
                }
                hm
            })
            .collect();

        // Plot phase losses
        if let Err(e) = plot_phase_losses(&phase_loss_data, &self.config.phase_loss_plot_path) {
            eprintln!("Failed to plot phase losses: {}", e);
        }

        self.save_model(trainer.model(), &self.config.model_path)
            .expect("Failed to save model.");
    }

    /// Loads the game dataset from the specified file.
    fn load_dataset(&self) -> (GameDataset, GameDataset) {
        GameDataset::load_auto(&self.config.dataset_base_path, self.config.train_ratio)
            .expect("Failed to load dataset")
    }

    /// Saves the trained model to a specified path
    pub fn save_model(&self, model: &Model, path: &str) -> std::io::Result<()> {
        model.save(path)?;
        println!("✅ Model saved at: {}", path);
        Ok(())
    }

    /// Loads the model from a specified path
    pub fn load_model(&self, path: &str) -> std::io::Result<Model> {
        let model = Model::load(path)?;
        println!("📥 Model loaded from {}", path);
        Ok(model)
    }
}
