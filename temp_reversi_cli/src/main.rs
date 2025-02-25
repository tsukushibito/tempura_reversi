use std::sync::Arc;

use clap::{Parser, Subcommand};
use temp_reversi_ai::learning::{TrainingConfig, TrainingPipeline};
use temp_reversi_cli::{
    run_test_match,
    utils::{GenerationReporter, TrainingReporter},
};

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

        // Path to model for self-play
        #[arg(short = 'm', long, default_value = "work/reversi_model.bin")]
        model_path: String,
    },

    /// Train the model
    Train {
        /// Path to load the dataset
        #[arg(short, long, default_value = "work/self_play_dataset")]
        dataset_base_path: String,

        /// Path to save the trained model
        #[arg(short, long, default_value = "work/reversi_model.bin")]
        model_path: String,

        /// Batch size for training
        #[arg(short, long, default_value = "64")]
        batch_size: usize,

        /// Number of epochs
        #[arg(short, long, default_value = "25")]
        epochs: usize,

        /// Path for overall loss plot
        #[arg(long, default_value = "work/loss_plot_overall.png")]
        overall_loss_plot_path: String,

        /// Path for phase loss plot
        #[arg(long, default_value = "work/loss_plot_phase.png")]
        phase_loss_plot_path: String,

        /// Learning rate for training
        #[arg(short = 'l', long, default_value = "0.0005")]
        learning_rate: f32,

        /// Training ratio
        #[arg(short = 't', long, default_value = "0.8")]
        train_ratio: f32,
    },

    /// Test match: games between PatternEvaluator and PhaseAwareEvaluator AIs.
    TestMatch {
        /// Number of games to play in test match (default: 100)
        #[arg(short, long, default_value = "100")]
        games: usize,

        /// Path to load the model (default: work/reversi_model.bin)
        #[arg(short, long, default_value = "gen0/model.bin")]
        black_model_path: String,

        #[arg(short, long, default_value = "gen0/gen0_model.bin")]
        white_model_path: String,
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
            model_path,
        } => {
            println!("🎯 Generating {} self-play games...", games);

            let generation_reporter = Arc::new(GenerationReporter::new());
            let config = TrainingConfig {
                num_games: games,
                batch_size: 32, // デフォルト値
                num_epochs: 10, // デフォルト値
                model_path,
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
        Commands::TestMatch {
            games,
            black_model_path,
            white_model_path,
        } => {
            println!(
                "Starting test match: {} games with model from {}...",
                games, black_model_path
            );
            run_test_match(games, &black_model_path, &white_model_path);
        }
    }
}
