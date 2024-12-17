use std::{
    fs::File,
    io::{Read, Write},
};

use reversi::{
    self_play, train_pattern_table, GameRecord, HyperParameter, PatternTable, SelfPlaySetting,
};

const RECORDS_FILE_PATH: &str = "records.bin";
const MODEL_FILE_PATH: &str = "model.bin";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    make_records()?;
    run_training()?;

    Ok(())
}

fn make_records() -> Result<(), Box<dyn std::error::Error>> {
    let setting = SelfPlaySetting {
        max_random_moves: 10,
        min_random_moves: 6,
        game_count: 100,
    };

    let records = self_play(&setting)?;

    let mut file = File::create(RECORDS_FILE_PATH)?;
    let encoded: Vec<u8> = bincode::serialize(&records)?;
    file.write_all(&encoded)?;

    Ok(())
}

fn run_training() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(RECORDS_FILE_PATH)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let records: Vec<GameRecord> = bincode::deserialize(&buffer)?;

    let hyper_param = HyperParameter {
        alpha: 0.01,
        max_iters: 1000,
        tolerance: 1e-6,
        batch_size: 64,
    };

    let mut pattern_table = PatternTable::default();
    train_pattern_table(&mut pattern_table, &records, &hyper_param);

    Ok(())
}
