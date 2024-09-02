use std::{fs::File, io::Write, task::Wake};

use burn::{
    backend::{Autodiff, Wgpu},
    optim::AdamConfig,
};
use clap::Parser;
use data::{make_game_records, ReversiItem};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use reversi::{self_play, BitBoard, Game, GameRecord, Model, Pattern, Position, SelfPlaySetting};

use crate::training::{train, TrainingConfig};

mod data;
mod model;
mod sparse_feature;
mod training;

type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'm', long, default_value_t = false)]
    pub make_game_records: bool,
}

fn main() -> DynResult<()> {
    let args = Args::parse();

    let artifact_dir = "tmp/training";
    let game_records_dir = "game_records";
    if args.make_game_records {
        make_game_records(game_records_dir)?;
        return Ok(());
    }

    type WgpuBackend = Wgpu<f32, i32>;
    type WgpuAutodiffBackend = Autodiff<WgpuBackend>;

    let device = burn::backend::wgpu::WgpuDevice::default();
    train::<WgpuAutodiffBackend>(
        artifact_dir,
        game_records_dir,
        TrainingConfig::new(AdamConfig::new()),
        device.clone(),
    );

    Ok(())
}
