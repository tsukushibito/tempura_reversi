use std::fs::File;

use burn::{
    backend::{Autodiff, Wgpu},
    optim::AdamConfig,
};
use clap::Parser;
use data::ReversiItem;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use reversi::{self_play, BitBoard, Game, GameRecord, Model, Pattern, Position, SelfPlaySetting};

use crate::training::{train, TrainingConfig};

mod data;
mod model;
mod training;

type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'm', long, default_value_t = false)]
    pub make_training_data: bool,
}

fn main() -> DynResult<()> {
    let args = Args::parse();

    let artifact_dir = "tmp/training";
    if args.make_training_data {
        make_training_data(1000, artifact_dir, "train")?;
        make_training_data(200, artifact_dir, "validation")?;
        make_training_data(200, artifact_dir, "test")?;
        return Ok(());
    }

    type WgpuBackend = Wgpu<f32, i32>;
    type WgpuAutodiffBackend = Autodiff<WgpuBackend>;

    let device = burn::backend::wgpu::WgpuDevice::default();
    train::<WgpuAutodiffBackend>(
        artifact_dir,
        TrainingConfig::new(AdamConfig::new()),
        device.clone(),
    );

    Ok(())
}

fn make_training_data(game_count: u64, artifact_dir: &str, file_name: &str) -> DynResult<()> {
    let pb = ProgressBar::new(game_count);
    pb.set_style(ProgressStyle::default_bar());

    let records: Vec<GameRecord> = (0..game_count)
        .into_par_iter()
        .progress_with(pb.clone())
        .map(|_| {
            let setting = SelfPlaySetting {
                max_random_moves: 10,
                min_random_moves: 6,
            };
            self_play(&setting)
        })
        .collect();

    let model = Model::default();
    let patterns = model.patterns();

    let items = extract_training_data(&records, patterns);

    let file = File::create(format!("{artifact_dir}/{file_name}"))?;
    let mut wtr = csv::Writer::from_writer(file);
    for item in items {
        wtr.serialize(item)?;
    }
    wtr.flush()?;

    Ok(())
}

fn extract_training_data(records: &[GameRecord], patterns: &[Pattern]) -> Vec<ReversiItem> {
    let mut training_data = Vec::new();

    for record in records {
        let diff = record.black_score as i32 - record.white_score as i32;
        let value = diff as f32;

        let mut game = Game::initial();

        for i in 0..=record.moves.len() {
            let board = BitBoard::from_board(game.board());

            let feature: Vec<f32> = patterns
                .iter()
                .flat_map(|pattern| pattern.feature(&board).into_iter())
                .collect();

            training_data.push(ReversiItem {
                feature_size: feature.len(),
                feature,
                value,
            });

            if i >= record.moves.len() {
                break;
            }

            let pos = Position::from_index(record.moves[i].into());
            let _ = game.progress(game.current_player(), pos);
        }
    }

    training_data
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_make_training_data() -> Result<(), Box<dyn std::error::Error>> {
        make_training_data(10, "test", "train")?;

        Ok(())
    }
}
