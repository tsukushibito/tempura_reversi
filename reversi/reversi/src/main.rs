use clap::{Parser, Subcommand};
use reversi::{generate_game_records, training, ResultBoxErr};

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
        #[arg(short, long, default_value = "records.bin")]
        output: String,
    },
    /// Train the AI model
    Train {
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
        Commands::Train { config } => {
            training(&config)?;
        }
    };

    Ok(())
}
