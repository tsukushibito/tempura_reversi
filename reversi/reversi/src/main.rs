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
        #[arg(short, long, default_value = "config.json")]
        config: String,
    },
    /// Train the AI model
    Train {
        #[arg(short, long, default_value = "config.json")]
        config: String,
    },
}

fn main() -> ResultBoxErr<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { config } => {
            generate_game_records(&config)?;
        }
        Commands::Train { config } => {
            training(&config)?;
        }
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> ResultBoxErr<()> {
        let config = "config.json";

        training(config)?;

        Ok(())
    }
}
