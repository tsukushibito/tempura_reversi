use std::{fs::File, io::BufReader};

use regex::Regex;
use serde::{Deserialize, Serialize};
use temp_reversi_core::{Game, Position};

use crate::dataset::ReversiSample;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GameRecord {
    /// Sequence of moves represented as board indices (0-63).
    pub moves: Vec<u8>,
    /// Final score of the game, represented as (black, white).
    pub final_score: (u8, u8),
}

// Define the maximum number of records per file
const MAX_RECORDS_PER_FILE: usize = 100_000;

impl GameRecord {
    pub fn load_records(
        dir: &str,
        base_file_name: &str,
    ) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let file_entries = Self::get_file_entries(dir, base_file_name)?;

        if file_entries.is_empty() {
            return Err("No game records found".into());
        }

        let results: Result<Vec<Vec<Self>>, Box<dyn std::error::Error>> = file_entries
            .into_iter()
            .map(|file_path| {
                println!("Loading file: {:?}", file_path);
                let file = File::open(&file_path)?;
                let mut reader = BufReader::new(file);

                let config = bincode::config::standard();
                let records: Vec<GameRecord> =
                    bincode::serde::decode_from_reader(&mut reader, config)?;

                Ok(records)
            })
            .collect();

        let all_records = results?.into_iter().flatten().collect::<Vec<_>>();

        Ok(all_records)
    }

    pub fn save_records(
        records: &[Self],
        dir: &str,
        base_file_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let dir_path = std::path::Path::new(dir);
        if !dir_path.exists() {
            std::fs::create_dir_all(dir_path)?;
        }

        let file_entries = Self::get_file_entries(dir, base_file_name)?;

        let next_index = if let Some(last_path) = file_entries.last() {
            let file_name = last_path.file_name().unwrap().to_string_lossy();
            let pattern = format!(r"^{}_(\d{{3}})\.bin$", base_file_name);
            let re = Regex::new(&pattern)?;

            if let Some(captures) = re.captures(&file_name) {
                let current_max = captures.get(1).unwrap().as_str().parse::<usize>()?;
                current_max + 1
            } else {
                0
            }
        } else {
            0
        };

        // Split records into chunks of 100,000 and save each chunk to a separate file
        for (chunk_index, chunk) in records.chunks(MAX_RECORDS_PER_FILE).enumerate() {
            let file_index = next_index + chunk_index;
            let file_path = dir_path.join(format!("{}_{:03}.bin", base_file_name, file_index));

            let mut file = File::create(file_path)?;
            let config = bincode::config::standard();
            bincode::serde::encode_into_std_write(chunk, &mut file, config)?;
        }

        Ok(())
    }

    pub fn to_samples(&self) -> Vec<ReversiSample> {
        let mut game = Game::default();
        let mut samples = Vec::new();

        for m in &self.moves {
            let pos = Position::from_u8(*m);
            let _ = game.apply_move(pos);
            let board = game.board_state();
            // let feature = extract_feature(board);
            // let packed_feature = FEATURE_PACKER.pack(&feature);
            let stone_diff = self.final_score.0 as i8 - self.final_score.1 as i8;
            // let sample = ReversiSample {
            //     indices: packed_feature.indices.to_vec(),
            //     phase: packed_feature.phase,
            //     stone_diff: label,
            // };
            let (black_bits, white_bits) = board.bits();
            let sample = ReversiSample {
                black_bits,
                white_bits,
                stone_diff,
            };
            samples.push(sample);
        }

        samples
    }

    fn get_file_entries(
        dir: &str,
        base_file_name: &str,
    ) -> Result<Vec<std::path::PathBuf>, Box<dyn std::error::Error>> {
        let dir = std::path::Path::new(dir);
        if !dir.exists() || !dir.is_dir() {
            return Ok(Default::default());
        }

        let pattern = format!(r"^{}_(\d{{3}})\.bin$", base_file_name);
        let re = Regex::new(&pattern)?;

        let mut file_entries: Vec<(usize, std::path::PathBuf)> = std::fs::read_dir(dir)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let file_name = entry.file_name().into_string().ok()?;

                if let Some(captures) = re.captures(&file_name) {
                    let num_str = captures.get(1)?.as_str();
                    if let Ok(num) = num_str.parse::<usize>() {
                        return Some((num, entry.path()));
                    }
                }

                None
            })
            .collect();

        file_entries.sort_by(|a, b| a.0.cmp(&b.0));
        let file_entries: Vec<std::path::PathBuf> =
            file_entries.into_iter().map(|(_, path)| path).collect();

        Ok(file_entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_game_records() {
        let records = vec![
            GameRecord {
                moves: vec![0, 1, 2],
                final_score: (10, 20),
            },
            GameRecord {
                moves: vec![3, 4, 5],
                final_score: (15, 25),
            },
        ];

        let dir = "test_records";
        let _ = std::fs::remove_dir_all(dir); // Clean up any existing test directory

        let base_file_name = "game_record";
        GameRecord::save_records(&records, dir, base_file_name).unwrap();

        // Load the records back to verify
        let loaded_records = GameRecord::load_records(dir, base_file_name).unwrap();

        let _ = std::fs::remove_dir_all(dir); // Clean up after test

        assert_eq!(loaded_records.len(), 2);
    }
}
