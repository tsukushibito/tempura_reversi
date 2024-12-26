use std::{
    fs::File,
    io::{Read, Write},
};

use burn::{
    data::{
        dataloader::batcher::Batcher,
        dataset::{Dataset, InMemDataset},
    },
    prelude::Backend,
    tensor::Tensor,
};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use reversi::{self_play, BitBoard, Game, GameRecord, Position, SelfPlaySetting};

use crate::{sparse_feature::SparseFeature, DynResult};

pub const TRAIN_GAME_RECORDS_FILE: &str = "train_gamerecords.bin";
pub const VALID_GAME_RECORDS_FILE: &str = "valid_gamerecords.bin";
pub const TEST_GAME_RECORDS_FILE: &str = "test_gamerecords.bin";

#[derive(Clone, Debug)]
pub struct ReversiItem {
    pub feature_size: usize,
    pub feature: SparseFeature,
    pub value: f32,
}

pub fn make_game_records(artifact_dir: &str) -> DynResult<()> {
    println!("making game records for train...");
    make_game_records_impl(1000, artifact_dir, TRAIN_GAME_RECORDS_FILE)?;

    println!("making game records for validation...");
    make_game_records_impl(200, artifact_dir, VALID_GAME_RECORDS_FILE)?;

    println!("making game records for test...");
    make_game_records_impl(200, artifact_dir, TEST_GAME_RECORDS_FILE)?;

    Ok(())
}

fn make_game_records_impl(game_count: u64, artifact_dir: &str, file_name: &str) -> DynResult<()> {
    let pb = ProgressBar::new(game_count);
    pb.set_style(ProgressStyle::default_bar());

    let map = (0..game_count)
        .into_par_iter()
        .progress_with(pb.clone())
        .map(|_| {
            let setting = SelfPlaySetting {
                max_random_moves: 10,
                min_random_moves: 6,
            };
            self_play(&setting)
        });
    let records: Vec<GameRecord> = map.collect();

    let buf = bincode::serialize(&records)?;

    std::fs::create_dir_all(artifact_dir)?;
    let mut file = File::create(format!("{artifact_dir}/{file_name}"))?;

    file.write_all(&buf)?;
    file.flush()?;

    Ok(())
}

fn load_game_records(artifact_dir: &str, file_name: &str) -> DynResult<Vec<GameRecord>> {
    let mut file = File::open(format!("{artifact_dir}/{file_name}"))?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let records: Vec<GameRecord> = bincode::deserialize(&buf)?;

    Ok(records)
}

fn make_items_from_game_records(records: &[GameRecord]) -> Vec<ReversiItem> {
    let model = reversi::Model::default();
    let patterns = model.patterns();

    let mut items = Vec::new();

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

            items.push(ReversiItem {
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

    items
}

pub struct ReversiDataset {
    dataset: InMemDataset<ReversiItem>,
}

impl Dataset<ReversiItem> for ReversiDataset {
    fn get(&self, index: usize) -> Option<ReversiItem> {
        self.dataset.get(index)
    }

    fn len(&self) -> usize {
        self.dataset.len()
    }
}

impl ReversiDataset {
    pub fn train(game_records_dir: &str) -> Option<Self> {
        println!("Loading game records for train...");
        let records = load_game_records(game_records_dir, TRAIN_GAME_RECORDS_FILE).ok()?;

        println!("Making items...");
        let items = make_items_from_game_records(&records);

        Some(Self::new(items))
    }

    pub fn validation(game_records_dir: &str) -> Option<Self> {
        println!("Loading game records for validation...");
        let records = load_game_records(game_records_dir, VALID_GAME_RECORDS_FILE).ok()?;

        println!("Making items...");
        let items = make_items_from_game_records(&records);

        Some(Self::new(items))
    }

    pub fn test(game_records_dir: &str) -> Option<Self> {
        println!("Loading game records for test...");
        let records = load_game_records(game_records_dir, TEST_GAME_RECORDS_FILE).ok()?;

        println!("Making items...");
        let items = make_items_from_game_records(&records);

        Some(Self::new(items))
    }

    pub fn new(items: Vec<ReversiItem>) -> Self {
        let dataset: InMemDataset<ReversiItem> = InMemDataset::new(items);
        Self { dataset }
    }

    pub fn d_input(&self) -> Option<usize> {
        let item = self.dataset.get(0)?;
        Some(item.feature_size)
    }
}

#[derive(Clone, Debug)]
pub struct ReversiBatcher<B: Backend> {
    device: B::Device,
}

impl<B: Backend> ReversiBatcher<B> {
    pub fn new(device: B::Device) -> Self {
        Self { device }
    }
}

#[derive(Clone, Debug)]
pub struct ReversiBatch<B: Backend> {
    pub inputs: Tensor<B, 2>,
    pub targets: Tensor<B, 1>,
}

impl<B: Backend> Batcher<ReversiItem, ReversiBatch<B>> for ReversiBatcher<B> {
    fn batch(&self, items: Vec<ReversiItem>) -> ReversiBatch<B> {
        let inputs = items
            .iter()
            .map(|item| {
                Tensor::<B, 1>::from_floats(item.feature.as_slice(), &self.device).unsqueeze()
            })
            .collect::<Vec<_>>();
        let inputs = Tensor::cat(inputs, 0).to_device(&self.device);

        let targets = items
            .iter()
            .map(|item| Tensor::<B, 1>::from_floats([item.value], &self.device))
            .collect::<Vec<_>>();
        let targets = Tensor::cat(targets, 0).to_device(&self.device);

        ReversiBatch { inputs, targets }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_make_game_records() -> Result<(), Box<dyn std::error::Error>> {
        make_game_records("test")?;

        Ok(())
    }

    #[test]
    fn test_make_items() -> Result<(), Box<dyn std::error::Error>> {
        let records = load_game_records("test", VALID_GAME_RECORDS_FILE)?;

        println!("Making items...");
        let _items = make_items_from_game_records(&records);

        Ok(())
    }

    #[test]
    fn test_reversidataset_train() -> Result<(), Box<dyn std::error::Error>> {
        let _ = ReversiDataset::train("test");

        Ok(())
    }

    #[test]
    fn test_reversidataset_validation() -> Result<(), Box<dyn std::error::Error>> {
        let _ = ReversiDataset::validation("test");

        Ok(())
    }

    #[test]
    fn test_reversidataset_test() -> Result<(), Box<dyn std::error::Error>> {
        let _ = ReversiDataset::test("test");

        Ok(())
    }
}
