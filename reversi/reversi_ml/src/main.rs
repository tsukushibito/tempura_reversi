use burn::{
    backend::{Autodiff, Wgpu},
    optim::AdamConfig,
};
use clap::Parser;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use reversi::{self_play, GameRecord, SelfPlaySetting};

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

    if args.make_training_data {
        make_training_data(1000)?;
        return Ok(());
    }

    type WgpuBackend = Wgpu<f32, i32>;
    type WgpuAutodiffBackend = Autodiff<WgpuBackend>;

    let device = burn::backend::wgpu::WgpuDevice::default();
    let artifact_dir = "/tmp/training";
    train::<WgpuAutodiffBackend>(
        artifact_dir,
        TrainingConfig::new(AdamConfig::new()),
        device.clone(),
    );

    Ok(())
}

fn make_training_data(game_count: u64) -> DynResult<()> {
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

    Ok(())
}
