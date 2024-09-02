use clap::{Parser, Subcommand};
use reversi::{eval_model, gen_data, training, ResultBoxErr};

#[derive(Parser)]
#[command(name = "Tempura Reversi")]
#[command(about = "An Othello AI CLI tool for data generation and training", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    GenData {
        #[arg(short, long, default_value = "config.json")]
        config: String,
    },
    Train {
        #[arg(short, long, default_value = "config.json")]
        config: String,
    },
    EvalModel {
        #[arg(short, long, default_value = "config.json")]
        config: String,
    },
}

fn main() -> ResultBoxErr<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenData { config } => {
            gen_data(&config)?;
        }
        Commands::Train { config } => {
            training(&config)?;
        }
        Commands::EvalModel { config } => {
            eval_model(&config)?;
        }
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> ResultBoxErr<()> {
        let config = "test_config.json";

        std::env::set_current_dir("reversi")?;

        training(config)?;

        Ok(())
    }
}
