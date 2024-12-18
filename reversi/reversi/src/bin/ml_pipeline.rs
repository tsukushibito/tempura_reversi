use std::{
    env::args,
    fs::File,
    io::{Read, Write},
};

use clap::Parser;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use reversi::{
    self_play, train_pattern_table, GameRecord, HyperParameter, PatternTable, SelfPlaySetting,
};
use serde::{de::DeserializeOwned, Serialize};

const RECORDS_FILE_PATH: &str = "records.bin";
const MODEL_FILE_PATH: &str = "model.bin";

type DynError = Box<dyn std::error::Error>;
type DynResult<T> = Result<T, DynError>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'R', long, default_value_t = false)]
    pub make_records: bool,

    #[arg(short = 'M', long, default_value_t = false)]
    pub make_model: bool,

    #[arg(short, long, default_value_t = false)]
    pub run_training: bool,

    #[arg(short, long, default_value_t = 0.01)]
    pub alpha: f32,

    #[arg(short = 'I', long, default_value_t = 200)]
    pub max_iters: usize,

    #[arg(short, long, default_value_t = 1e-6)]
    pub tolerance: f32,

    #[arg(short, long, default_value_t = 64)]
    pub batch_size: usize,

    #[arg(short, long, default_value_t = 0.9)]
    pub beta1: f32,

    #[arg(short, long, default_value_t = 0.999)]
    pub beta2: f32,

    #[arg(short, long, default_value_t = 1e-8)]
    pub epsilon: f32,
}

fn main() -> DynResult<()> {
    let args = Args::parse();

    if args.make_records {
        make_records(1000)?;
    }

    if args.make_model {
        make_model()?;
    }

    if args.run_training {
        let mut model = load_model()?;
        let records = load_records()?;
        let hyper_param = HyperParameter {
            alpha: args.alpha,
            max_iters: args.max_iters,
            tolerance: args.tolerance,
            batch_size: args.batch_size,
            beta1: args.beta1,
            beta2: args.beta2,
            epsilon: args.epsilon,
        };
        run_training(&mut model, &records, &hyper_param)?;

        save_model(&model)?;
    }

    Ok(())
}

fn load_data<T>(file_path: &str) -> DynResult<T>
where
    T: DeserializeOwned,
{
    let mut file = File::open(file_path)?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let data: T = bincode::deserialize(&buf)?;

    Ok(data)
}

fn save_data<T>(data: &T, file_path: &str) -> DynResult<()>
where
    T: Serialize,
{
    let buf = bincode::serialize(data)?;
    let mut file = File::create(file_path)?;
    file.write_all(&buf)?;

    Ok(())
}

fn load_records() -> DynResult<Vec<GameRecord>> {
    load_data(RECORDS_FILE_PATH)
}

fn save_records(records: &[GameRecord]) -> DynResult<()> {
    save_data(&records, RECORDS_FILE_PATH)
}

fn make_records(game_count: u64) -> DynResult<()> {
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

    save_records(&records)
}

fn load_model() -> DynResult<PatternTable> {
    load_data(MODEL_FILE_PATH)
}

fn save_model(model: &PatternTable) -> DynResult<()> {
    save_data(model, MODEL_FILE_PATH)
}

fn make_model() -> DynResult<()> {
    let model = PatternTable::default();

    save_model(&model)
}

fn run_training(
    model: &mut PatternTable,
    records: &[GameRecord],
    hyper_param: &HyperParameter,
) -> DynResult<()> {
    train_pattern_table(model, records, hyper_param);

    Ok(())
}
