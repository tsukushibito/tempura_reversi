use clap::{Parser, Subcommand};
use reversi::{gen_data_for_training, gen_data_for_validation, training, ResultBoxErr};

#[derive(Parser)]
#[command(name = "Tempura Reversi")]
#[command(about = "An Othello AI CLI tool for data generation and training", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    GenTrainData {
        #[arg(short, long, default_value = "config.json")]
        config: String,
    },
    GenValidData {
        #[arg(short, long, default_value = "config.json")]
        config: String,
    },
    Train {
        #[arg(short, long, default_value = "config.json")]
        config: String,
    },
}

fn main() -> ResultBoxErr<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenTrainData { config } => {
            gen_data_for_training(&config)?;
        }
        Commands::GenValidData { config } => {
            gen_data_for_validation(&config)?;
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
