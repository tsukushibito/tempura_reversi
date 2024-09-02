use std::{
    fs::File,
    io::{stdin, Write},
    path::{Path, PathBuf},
};

use indicatif::ProgressBar;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    ml::{self_play, GameRecord, SelfPlaySetting},
    Config, ResultBoxErr,
};

pub fn gen_data(config: &str) -> ResultBoxErr<()> {
    let config = Config::from_file(config)?;

    let output = config.gen_data_train_path();
    println!("Generating data for training...");
    gen_data_impl(&output, config.gen_data.num_games_for_train)?;

    let output = config.gen_data_valid_path();
    println!("Generating data for validation...");
    gen_data_impl(&output, config.gen_data.num_games_for_valid)?;

    Ok(())
}

fn gen_data_impl(output: &PathBuf, num_games: usize) -> ResultBoxErr<()> {
    let pb = ProgressBar::new(num_games.try_into().unwrap());
    let records: Vec<GameRecord> = (0..num_games)
        .into_par_iter()
        .map(|_| {
            let setting = SelfPlaySetting {
                max_random_moves: 10,
                min_random_moves: 6,
            };
            let record = self_play(&setting);
            pb.inc(1);

            record
        })
        .collect();

    let path = Path::new(&output);
    if path.exists() {
        println!(
            "ファイル '{}' は既に存在します。上書きしますか？ (y/n): ",
            output.display()
        );

        // ユーザー入力を受け取る
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        if input != "y" && input != "yes" {
            println!("上書きをキャンセルしました。");
            return Ok(());
        }
    }

    let mut file = File::create(output)?;
    let serialized = bincode::serialize(&records)?;
    file.write_all(&serialized)?;
    file.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_game_records() -> ResultBoxErr<()> {
        let cwd = std::env::current_dir().unwrap();
        println!("Current working directory: {:?}", cwd);

        let new_dir = std::path::Path::new("reversi");
        if let Err(e) = std::env::set_current_dir(new_dir) {
            eprintln!("Failed to change directory: {}", e);
        }

        Ok(())
    }
}
