use std::{
    fs::File,
    io::{stdin, Write},
    path::Path,
};

use indicatif::ProgressBar;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{self_play, GameRecord, ResultBoxErr, SelfPlaySetting};

pub fn generate_game_records(output: &str) -> ResultBoxErr<()> {
    let game_count = 1000;
    let pb = ProgressBar::new(game_count);
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
            output
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
