use clap::{Parser, Subcommand};
use reversi::{generate_game_records, ResultBoxErr};

#[derive(Parser)]
#[command(name = "Tempura Reversi")]
#[command(about = "An Othello AI CLI tool for data generation and training", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate game record
    Generate {
        /// Output file or directory for generated data
        #[arg(short, long, default_value = "output.bin")]
        output: String,
    },
    /// Train the AI model
    Train {
        /// Number of training iterations
        #[arg(short, long, default_value_t = 1000)]
        iterations: u32,

        /// Path to the configuration file
        #[arg(short, long, default_value = "config.json")]
        config: String,
    },
}

fn main() -> ResultBoxErr<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { output } => {
            println!("Generating game record to: {}", output);
            generate_game_records(&output)?;
        }
        Commands::Train { iterations, config } => {
            println!(
                "Training AI with {} iterations using config: {}",
                iterations, config
            );
            // 学習処理
        }
    };

    Ok(())
}
