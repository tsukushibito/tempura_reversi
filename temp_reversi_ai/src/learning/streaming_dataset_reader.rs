use rayon::prelude::*;
use std::path::Path;

use crate::{
    patterns::{get_predefined_patterns, PatternGroup},
    utils::Feature,
};

use super::{Dataset, GameDataset, GameRecord};

/// StreamingDatasetReader automatically detects the corresponding
/// file (either a single file or split files) based on the given base_file_name,
/// loads them on demand, and returns an iterator that yields Dataset batches.
///
/// # Example
///
/// ```rust
/// use temp_reversi_ai::learning::StreamingDatasetReader;  // adjust the import path as needed
///
/// let mut reader = StreamingDatasetReader::new("data/file", 32);
/// for batch in reader {
///     // process the batch
/// }
/// ```
///
pub struct StreamingDatasetReader {
    file_paths: Vec<String>,
    current_file_index: usize,
    current_records: Option<Vec<GameRecord>>,
    record_cursor: usize,
    batch_size: usize,
    pattern_groups: Vec<PatternGroup>,
}

impl StreamingDatasetReader {
    /// Creates a new reader with the given base_file_name and batch size.
    /// Files are detected as either "base_file_name.bin" or "base_file_name_part_X.bin".
    pub fn new(base_file_name: &str, batch_size: usize) -> Self {
        let mut file_paths = Vec::new();
        // For the single file case
        let base_path = format!("{}.bin", base_file_name);
        if Path::new(&base_path).exists() {
            file_paths.push(base_path);
        } else {
            // For the split files case: base_file_name_part_1.bin, base_file_name_part_2.bin, ...
            let mut part = 1;
            loop {
                let part_path = format!("{}_part_{}.bin", base_file_name, part);
                if Path::new(&part_path).exists() {
                    file_paths.push(part_path);
                    part += 1;
                } else {
                    break;
                }
            }
        }
        Self {
            file_paths,
            current_file_index: 0,
            current_records: None,
            record_cursor: 0,
            batch_size,
            pattern_groups: get_predefined_patterns(),
        }
    }

    fn load_next_file(&mut self) -> Option<()> {
        if self.current_file_index >= self.file_paths.len() {
            return None;
        }
        let file_path = &self.file_paths[self.current_file_index];
        match GameDataset::load_bin(file_path) {
            Ok(dataset) => {
                self.current_records = Some(dataset.records);
                self.record_cursor = 0;
                self.current_file_index += 1;
                Some(())
            }
            Err(_) => None,
        }
    }
}

impl Iterator for StreamingDatasetReader {
    type Item = Dataset;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // If the current file is not loaded, load the next one
            if self.current_records.is_none() {
                if self.load_next_file().is_none() {
                    return None; // All files have been processed
                }
            }
            let records = self.current_records.as_mut().unwrap();
            if self.record_cursor >= records.len() {
                self.current_records = None;
                continue;
            }
            // Retrieve records up to the batch size
            let end = (self.record_cursor + self.batch_size).min(records.len());
            let batch_records = &records[self.record_cursor..end];
            self.record_cursor = end;

            // Process each record using the existing process_record to produce a Dataset
            let samples: Vec<(Feature, f32)> = batch_records
                .par_iter()
                .flat_map(|record| GameDataset::process_record(record, &self.pattern_groups))
                .collect();
            let mut batch = Dataset::new();
            for (feature, label) in samples {
                batch.add_sample(feature, label);
            }
            return Some(batch);
        }
    }
}
