use std::{
    fs::File,
    io::{Read, Write},
};

use reversi::{self_play, GameRecord, SelfPlaySetting};

const FILE_PATH: &str = "records.bint";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    make_records()?;
    run_training()?;

    Ok(())
}

fn make_records() -> Result<(), Box<dyn std::error::Error>> {
    let setting = SelfPlaySetting {
        max_random_moves: 10,
        min_random_moves: 6,
        game_count: 10,
    };

    let records = self_play(&setting)?;

    let mut file = File::create(FILE_PATH)?;
    let encoded: Vec<u8> = bincode::serialize(&records)?;
    file.write_all(&encoded)?;

    Ok(())
}

fn run_training() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(FILE_PATH)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let records: Vec<GameRecord> = bincode::deserialize(&buffer)?;

    Ok(())
}
