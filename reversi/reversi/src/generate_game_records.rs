use std::{
    fs::File,
    io::{stdin, Write},
    path::Path,
};

use indicatif::ProgressBar;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    ml::{self_play, GameRecord, SelfPlaySetting},
    Config, ResultBoxErr,
};

pub fn generate_game_records(config: &str) -> ResultBoxErr<()> {
    let config = Config::from_file(config)?;
    let output = config.self_play_output_path();

    let game_count = config.self_play.num_games;
    let pb = ProgressBar::new(game_count.try_into().unwrap());
    let records: Vec<GameRecord> = (0..game_count)
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
