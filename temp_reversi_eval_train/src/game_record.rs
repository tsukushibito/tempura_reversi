use std::{
    fs::File,
    io::{BufReader, Write},
};

use serde::{Deserialize, Serialize};
use temp_reversi_core::{Game, Position};
use temp_reversi_eval::feature::extract_feature;

use crate::dataset::ReversiSample;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameRecordsFileHeader {
    pub record_count: u32,
    pub move_count: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GameRecord {
    /// Sequence of moves represented as board indices (0-63).
    pub moves: Vec<u8>,
    /// Final score of the game, represented as (black, white).
    pub final_score: (u8, u8),
}

impl GameRecord {
    pub fn load_records(base_path: &str, file_index: usize) -> (GameRecordsFileHeader, Vec<Self>) {
        let file_entries = get_file_entries(base_path);
        if file_index >= file_entries.len() {
            panic!("File index {} out of bounds", file_index);
        }
        let (_, file_path) = file_entries[file_index].clone();

        println!("Loading file: {:?}", file_path);
        let file = File::open(&file_path).expect("Failed to open file");
        let mut reader = BufReader::new(file);

        let config = bincode::config::standard();
        let header: GameRecordsFileHeader =
            bincode::serde::decode_from_reader(&mut reader, config).expect("Failed to read header");
        let records: Vec<GameRecord> = bincode::serde::decode_from_reader(&mut reader, config)
            .expect("Failed to read game records");

        if records.len() != header.record_count as usize {
            println!(
                "[Warning] Mismatch between header record count and actual records: {} vs {}",
                header.record_count,
                records.len()
            );
        }

        (header, records)
    }

    pub fn save_records(records: &[Self], base_path: &str) {
        let file_entries = get_file_entries(base_path);
        let new_index = file_entries.last().map(|(idx, _)| idx + 1).unwrap_or(0);

        let base = std::path::Path::new(base_path);
        let parent = base.parent().unwrap_or_else(|| std::path::Path::new("."));
        let base_stem = base
            .file_stem()
            .and_then(|s| s.to_str())
            .expect("Invalid file stem");
        let extension = base
            .extension()
            .and_then(|s| s.to_str())
            .expect("Invalid file extension");

        // If parent is not a directory, create it
        if !parent.exists() {
            std::fs::create_dir_all(parent).expect("Failed to create directory");
        }

        // Construct new file name: for index 0, you might want to use the base file name,
        // but for appended files (index > 0) use the numeric suffix.
        let new_file_name = if new_index == 0 {
            base.file_name().expect("Invalid base file name").to_owned()
        } else {
            std::ffi::OsString::from(format!(
                "{}_{}.{}",
                base_stem,
                format!("{:03}", new_index),
                extension
            ))
        };
        let new_file_path = parent.join(new_file_name);

        println!("Saving records to {:?}", new_file_path);
        let mut file = std::fs::File::create(&new_file_path).expect("Failed to create file");

        let header = GameRecordsFileHeader {
            record_count: records.len() as u32,
            move_count: records.iter().map(|r| r.moves.len()).sum::<usize>() as u32,
        };

        let config = bincode::config::standard();
        let encoded_header =
            bincode::serde::encode_to_vec(header, config).expect("Failed to write header");
        let encoded_records =
            bincode::serde::encode_to_vec(records, config).expect("Failed to write game records");
        file.write_all(&encoded_header)
            .expect("Failed to write header");
        file.write_all(&encoded_records)
            .expect("Failed to write game records");
    }

    pub fn to_samples(&self) -> Vec<ReversiSample> {
        let mut game = Game::default();
        let mut samples = Vec::new();

        for m in &self.moves {
            let pos = Position::from_u8(*m);
            let _ = game.apply_move(pos);
            let board = game.board_state();
            let feature = extract_feature(board);
            let label = self.final_score.0 as i8 - self.final_score.1 as i8;
            let sample = ReversiSample {
                feature,
                stone_diff: label,
            };
            samples.push(sample);
        }

        samples
    }
}

fn get_file_entries(base_path: &str) -> Vec<(usize, std::path::PathBuf)> {
    let base_path = std::path::Path::new(base_path);
    let dir = base_path
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."));
    let base_file_name = base_path
        .file_name()
        .and_then(|s| s.to_str())
        .expect("Invalid base file name");
    let base_stem = base_path
        .file_stem()
        .and_then(|s| s.to_str())
        .expect("Invalid file stem");
    let extension = base_path
        .extension()
        .and_then(|s| s.to_str())
        .expect("Invalid file extension");

    // If dir is not exists, return empty vector
    if !dir.exists() {
        return Vec::new();
    }

    let mut file_entries: Vec<(usize, std::path::PathBuf)> = std::fs::read_dir(dir)
        .expect("Failed to read base directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let file_name = entry.file_name().into_string().ok()?;
            // Case 1: When only one file exists, it will match exactly the base file name.
            if file_name == base_file_name {
                return Some((0, entry.path()));
            }
            // Case 2: For multiple files using the numeric suffix convention: "{base_stem}_NNN.{extension}"
            let prefix = format!("{}_", base_stem);
            let suffix = format!(".{}", extension);
            if file_name.starts_with(&prefix) && file_name.ends_with(&suffix) {
                let start = prefix.len();
                let end = file_name.len() - suffix.len();
                let number_str = &file_name[start..end];
                if let Ok(num) = number_str.parse::<usize>() {
                    return Some((num, entry.path()));
                }
            }
            None
        })
        .collect();

    file_entries.sort_by_key(|&(num, _)| num);
    file_entries
}

fn get_file_headers(base_path: &str) -> Vec<GameRecordsFileHeader> {
    let file_entries = get_file_entries(base_path);
    file_entries
        .iter()
        .map(|(_, path)| {
            let file = File::open(path).expect("Failed to open file");
            let mut reader = BufReader::new(file);
            let header: GameRecordsFileHeader =
                bincode::serde::decode_from_reader(&mut reader, bincode::config::standard())
                    .expect("Failed to read header");
            header
        })
        .collect()
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

        let base_path = "test_records/game_records.bin";
        GameRecord::save_records(&records, base_path);

        // Load the records back to verify
        let (_header, loaded_records) = GameRecord::load_records(base_path, 0);
        assert_eq!(loaded_records.len(), records.len());
    }
}
