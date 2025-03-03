use std::collections::HashMap;
use std::sync::Arc;

use crate::evaluator::TempuraEvaluator;
use crate::learning::loss_function::MSELoss;
use crate::learning::optimizer::Adam;
use crate::learning::{extract_features, generate_game_dataset, Trainer};
use crate::patterns::get_predefined_patterns;
use crate::plotter::{plot_overall_loss, plot_phase_losses};
use crate::strategy::NegaAlphaTTStrategy;
use crate::utils::ProgressReporter;

use super::{Model, StreamingDatasetWriter};

/// Configuration for the training pipeline.
pub struct TrainingConfig {
    /// Number of self-play games to generate.
    pub num_train_games: usize,
    pub num_validation_games: usize,
    pub init_random_moves: usize,
    /// Batch size for training.
    pub batch_size: usize,
    /// Number of epochs for model training.
    pub num_epochs: usize,
    /// Path to save the trained model.
    pub model_path: String,
    /// Path to save the generated game dataset.
    pub train_dataset_base_path: String,
    pub validation_dataset_base_path: String,
    /// Path for overall loss plot.
    pub overall_loss_plot_path: String,
    /// Path for phase loss plot.
    pub phase_loss_plot_path: String,
    /// Learning rate for the optimizer.
    pub learning_rate: f32,
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
        self.generate_dataset(None);
        self.train(None);
    }

    /// Generates self-play data using AI strategies and saves it to a file.
    pub fn generate_dataset(&self, reporter: Option<Arc<dyn ProgressReporter + Send + Sync>>) {
        self.generate_dataset_impl(
            &self.config.train_dataset_base_path,
            self.config.num_train_games,
            reporter.clone(),
        );
        self.generate_dataset_impl(
            &self.config.validation_dataset_base_path,
            self.config.num_validation_games,
            reporter,
        );
    }

    /// Loads the dataset and trains the model.
    pub fn train(&self, reporter: Option<Arc<dyn ProgressReporter + Send + Sync>>) {
        // Generate a dummy board state to extract feature vector and determine its size.
        let dummy_board = temp_reversi_core::Bitboard::default();
        let groups = get_predefined_patterns();
        let feature_vector = extract_features(&dummy_board, &groups);
        let feature_size = feature_vector.size();
        let optimizer = Adam::new(feature_size, self.config.learning_rate, 0.001, 0.001);
        let mut trainer = Trainer::new(
            feature_size,
            MSELoss,
            optimizer,
            self.config.batch_size,
            self.config.num_epochs,
            Some(&self.config.model_path),
        );

        // Pass reporter if available; here using None. Replace with Some(reporter) as needed.
        trainer.train(
            &self.config.train_dataset_base_path,
            &self.config.validation_dataset_base_path,
            reporter,
        );

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

    fn generate_dataset_impl(
        &self,
        dataset_base_path: &str,
        num_games: usize,
        reporter: Option<Arc<dyn ProgressReporter + Send + Sync>>,
    ) {
        let tempura_evaluator = TempuraEvaluator::new(&self.config.model_path);
        let mut writer = StreamingDatasetWriter::new(dataset_base_path, 100000);
        let mut remain_games = num_games;
        while remain_games > 0 {
            let num_games = remain_games.min(100000);
            println!("Generating {}/{} games...", num_games, remain_games);
            let game_dataset = generate_game_dataset(
                num_games,
                Box::new(NegaAlphaTTStrategy::new(tempura_evaluator.clone(), 5, 0.0)),
                self.config.init_random_moves,
                reporter.clone(),
            );

            game_dataset.records.into_iter().for_each(|record| {
                writer.add_record(record).expect("Failed to add record.");
            });

            remain_games -= num_games;
        }

        writer.flush().expect("Failed to flush writer.");
    }
}
