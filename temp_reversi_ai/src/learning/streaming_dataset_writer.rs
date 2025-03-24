use lz4_flex::compress_prepend_size;
use std::fs;
use std::path::Path;

use super::{GameDataset, GameRecord};

pub struct StreamingDatasetWriter {
    base_file_name: String,
    max_records_per_file: usize,
    current_records: Vec<GameRecord>,
    part_index: usize,
}

impl StreamingDatasetWriter {
    /// Creates a new writer with the given base_file_name and maximum number of records per file.
    pub fn new(base_file_name: &str, max_records_per_file: usize) -> Self {
        Self {
            base_file_name: base_file_name.to_string(),
            max_records_per_file,
            current_records: Vec::new(),
            part_index: 0,
        }
    }

    /// Adds a new GameRecord.
    /// Automatically flushes to file if the internal buffer reaches the maximum size.
    pub fn add_record(&mut self, record: GameRecord) -> std::io::Result<()> {
        self.current_records.push(record);
        if self.current_records.len() >= self.max_records_per_file {
            self.flush_current()?;
        }
        Ok(())
    }

    fn flush_current(&mut self) -> std::io::Result<()> {
        if self.current_records.is_empty() {
            return Ok(());
        }
        // For a single file, use "base_file_name.bin". If split into multiple files, use "base_file_name_part_X.bin"
        let file_path =
            if self.part_index == 0 && self.current_records.len() < self.max_records_per_file {
                format!("{}.bin", self.base_file_name)
            } else {
                format!("{}_part_{}.bin", self.base_file_name, self.part_index + 1)
            };

        // Temporarily create a GameDataset to reuse the existing save logic.
        let dataset = GameDataset {
            records: std::mem::take(&mut self.current_records),
        };
        let encoded: Vec<u8> = bincode::serialize(&dataset).unwrap();
        let compressed = compress_prepend_size(&encoded);
        if let Some(parent) = Path::new(&file_path).parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        fs::write(&file_path, compressed)?;
        self.part_index += 1;
        Ok(())
    }

    /// Flushes any remaining records in the buffer to a file.
    pub fn flush(&mut self) -> std::io::Result<()> {
        self.flush_current()
    }
}
