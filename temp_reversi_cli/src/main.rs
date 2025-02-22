use std::sync::Arc;

use clap::{Parser, Subcommand};
use temp_reversi_ai::learning::{TrainingConfig, TrainingPipeline};
use temp_reversi_cli::utils::{GenerationReporter, TrainingReporter};

#[derive(Parser)]
#[command(name = "reversi-cli")]
#[command(about = "Reversi CLI for training and self-play", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a new game
    Play,

    /// Generate self-play data
    Generate {
        /// Number of games to generate
        #[arg(short, long, default_value = "10000")]
        games: usize,

        /// Path to save the generated dataset
        #[arg(short = 'o', long, default_value = "work/self_play_dataset")]
        dataset_base_path: String,
    },

    /// Train the model
    Train {
        /// Path to load the dataset
        #[arg(short, long, default_value = "work/self_play_dataset")]
        dataset_base_path: String,

        /// Path to save the trained model
        #[arg(short = 'o', long, default_value = "work/reversi_model.bin")]
        model_path: String,

        /// Batch size for training
        #[arg(short, long, default_value = "32")]
        batch_size: usize,

        /// Number of epochs
        #[arg(short, long, default_value = "16")]
        epochs: usize,

        /// Path for overall loss plot
        #[arg(long, default_value = "work/loss_plot_overall.png")]
        overall_loss_plot_path: String,

        /// Path for phase loss plot
        #[arg(long, default_value = "work/loss_plot_phase.png")]
        phase_loss_plot_path: String,

        /// Learning rate for training
        #[arg(short = 'l', long, default_value = "0.0004")]
        learning_rate: f32,

        /// Training ratio
        #[arg(short = 't', long, default_value = "0.8")]
        train_ratio: f32,
    },
}

fn main() {
    env_logger::init(); // ✅ ログ出力の初期化
    let cli = Cli::parse();

    match cli.command {
        Commands::Play => {
            println!("Starting a new game...");
            // ゲームロジックをここに実装
        }
        Commands::Generate {
            games,
            dataset_base_path: dataset_path,
        } => {
            println!("🎯 Generating {} self-play games...", games);

            let generation_reporter = Arc::new(GenerationReporter::new());
            let config = TrainingConfig {
                num_games: games,
                batch_size: 32, // デフォルト値
                num_epochs: 10, // デフォルト値
                model_path: "reversi_model.bin".to_string(),
                dataset_base_path: dataset_path,
                train_ratio: 0.8,
                overall_loss_plot_path: Default::default(),
                phase_loss_plot_path: Default::default(),
                learning_rate: 0.0001,
            };

            let pipeline = TrainingPipeline::new(config);
            pipeline.generate_self_play_data(Some(generation_reporter));

            println!("✅ Data generation completed.");
        }
        Commands::Train {
            dataset_base_path: dataset_path,
            model_path,
            batch_size,
            epochs,
            overall_loss_plot_path,
            phase_loss_plot_path,
            learning_rate,
            train_ratio,
        } => {
            println!("📊 Starting training with dataset: {}", dataset_path);

            let training_reporter = Arc::new(TrainingReporter::new());

            let config = TrainingConfig {
                num_games: 0,
                batch_size,
                num_epochs: epochs,
                model_path,
                dataset_base_path: dataset_path,
                train_ratio, // updated value from CLI argument
                overall_loss_plot_path,
                phase_loss_plot_path,
                learning_rate, // new field added
            };

            let pipeline = TrainingPipeline::new(config);
            pipeline.train(Some(training_reporter));

            println!("✅ Model training completed.");
        }
    }
}
