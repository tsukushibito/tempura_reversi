use std::sync::Arc;

use clap::{Parser, Subcommand};
use temp_reversi_ai::learning::{TrainingConfig, TrainingPipeline};
use temp_reversi_cli::{
    run_test_match, shuffle_dataset,
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
        #[arg(long, default_value = "1000000")]
        num_train_games: usize,

        /// Number of games to generate
        #[arg(long, default_value = "50000")]
        num_validation_games: usize,

        #[arg(short, long, default_value = "10")]
        init_random_moves: usize,

        /// Path to save the generated dataset
        #[arg(short, long, default_value = "gen0/dataset/temp_dataset")]
        train_dataset_base_path: String,

        #[arg(short, long, default_value = "gen0/dataset/temp_validation_dataset")]
        validation_dataset_base_path: String,

        // Path to model for self-play
        #[arg(short, long, default_value = "gen0/models/temp_model.bin")]
        model_path: String,
    },

    /// Train the model
    Train {
        /// Path to load the dataset
        #[arg(short, long, default_value = "gen0/dataset/temp_dataset")]
        train_dataset_base_path: String,

        #[arg(short, long, default_value = "gen0/dataset/temp_validation_dataset")]
        validation_dataset_base_path: String,

        /// Path to save the trained model
        #[arg(short, long, default_value = "gen0/models/temp_model.bin")]
        model_path: String,

        /// Batch size for training
        #[arg(short, long, default_value = "1024")]
        batch_size: usize,

        /// Number of epochs
        #[arg(short, long, default_value = "25")]
        epochs: usize,

        /// Path for overall loss plot
        #[arg(long, default_value = "gen0/loss_plot_overall.png")]
        overall_loss_plot_path: String,

        /// Path for phase loss plot
        #[arg(long, default_value = "gen0/loss_plot_phase.png")]
        phase_loss_plot_path: String,

        /// Learning rate for training
        #[arg(short = 'l', long, default_value = "0.0005")]
        learning_rate: f32,
    },

    /// Test match: games between PatternEvaluator and PhaseAwareEvaluator AIs.
    TestMatch {
        /// Number of games to play in test match
        #[arg(short, long, default_value = "100")]
        games: usize,

        /// Path to load the model
        #[arg(short, long, default_value = "gen0/model.bin")]
        black_model_path: String,

        #[arg(short, long, default_value = "gen0/gen0_model.bin")]
        white_model_path: String,
    },

    // Shuffle the dataset
    Shuffle {
        #[arg(short, long, default_value = "gen0/dataset/dataset")]
        dataset_base_path: String,

        #[arg(short, long, default_value = "gen0/dataset/dataset")]
        outpu_dataset_base_path: String,
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
            num_train_games,
            num_validation_games,
            init_random_moves,
            train_dataset_base_path,
            validation_dataset_base_path,
            model_path,
        } => {
            println!("🎯 Generating {} train games...", num_train_games);

            let generation_reporter = Arc::new(GenerationReporter::new());
            let config = TrainingConfig {
                num_train_games,
                num_validation_games,
                init_random_moves,
                batch_size: 32, // デフォルト値
                num_epochs: 10, // デフォルト値
                model_path,
                train_dataset_base_path,
                validation_dataset_base_path,
                overall_loss_plot_path: Default::default(),
                phase_loss_plot_path: Default::default(),
                learning_rate: 0.0001,
            };

            let pipeline = TrainingPipeline::new(config);
            pipeline.generate_dataset(Some(generation_reporter));

            println!("✅ Data generation completed.");
        }
        Commands::Train {
            train_dataset_base_path,
            validation_dataset_base_path,
            model_path,
            batch_size,
            epochs,
            overall_loss_plot_path,
            phase_loss_plot_path,
            learning_rate,
        } => {
            println!(
                "📊 Starting training with dataset: {}",
                train_dataset_base_path
            );

            let training_reporter = Arc::new(TrainingReporter::new());

            let config = TrainingConfig {
                num_train_games: 0,
                num_validation_games: 0,
                init_random_moves: 0,
                batch_size,
                num_epochs: epochs,
                model_path,
                train_dataset_base_path,
                validation_dataset_base_path,
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
        Commands::Shuffle {
            dataset_base_path,
            outpu_dataset_base_path,
        } => {
            shuffle_dataset(&dataset_base_path, &outpu_dataset_base_path)
                .expect("Failed to shuffle dataset");
        }
    }
}
