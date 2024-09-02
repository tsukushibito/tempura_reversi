use std::{fs::File, io::Read, path::Path};

use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

use crate::{
    ml::GameRecord, BitBoard, Game, Position, ResultBoxErr, SparseVector, TempuraEvaluator,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataItem {
    pub feature: SparseVector,
    pub target: f32,
}

pub fn get_data_items_from_record(record: &GameRecord) -> Vec<DataItem> {
    let evaluator = TempuraEvaluator::default();
    let mut game = Game::initial();
    let mut data_items = vec![];
    let target = record.black_score as f32 - record.white_score as f32;

    for &mov in &record.moves {
        let player = game.current_player();
        let _ = game.progress(player, Position::from_index(mov.into()));
        let board = game.board();
        let bit_board = BitBoard::from_board(board);
        let feature = evaluator.feature(&bit_board);
        data_items.push(DataItem { feature, target });
    }

    data_items
}

pub fn transpose<T>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if matrix.is_empty() {
        return Vec::new();
    }

    let col_count = matrix[0].len();

    for row in &matrix {
        assert_eq!(row.len(), col_count, "行列が矩形ではありません。");
    }

    let row_count = matrix.len();

    let mut transposed = Vec::with_capacity(col_count);
    for _ in 0..col_count {
        transposed.push(Vec::with_capacity(row_count));
    }

    for row in matrix {
        for (j, item) in row.into_iter().enumerate() {
            transposed[j].push(item);
        }
    }

    transposed
}

#[derive(Debug, Clone)]
pub struct Dataloader {
    records: Vec<GameRecord>,
    batch_size: usize,
    current_index: usize,
}

impl Dataloader {
    pub fn from_data_file<P: AsRef<Path>>(
        data_file_path: P,
        batch_size: usize,
    ) -> ResultBoxErr<Self> {
        println!(
            "[Dataloader::from_file()] data_file_path={:?}",
            data_file_path.as_ref().to_str().unwrap()
        );

        let data_file_path: String = data_file_path
            .as_ref()
            .to_str()
            .ok_or("invalid path")?
            .to_string();

        let mut file = File::open(&data_file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let records: Vec<GameRecord> = bincode::deserialize(&buffer)?;

        Ok(Self {
            records,
            batch_size,
            current_index: 0,
        })
    }

    pub fn next_batch(&mut self) -> Option<&[GameRecord]> {
        if self.current_index >= self.records.len() {
            return None;
        }

        let end = std::cmp::min(self.current_index + self.batch_size, self.records.len());
        let batch = &self.records[self.current_index..end];
        self.current_index = end;
        Some(batch)
    }

    pub fn reset(&mut self) -> ResultBoxErr<()> {
        let mut rng = thread_rng();
        self.records.shuffle(&mut rng);

        self.current_index = 0;

        Ok(())
    }

    pub fn iter_batches(&self) -> DataloaderIterator {
        DataloaderIterator {
            records: &self.records,
            batch_size: self.batch_size,
            current_index: 0,
        }
    }

    pub fn batch_count(&self) -> usize {
        self.records.len() / self.batch_size
    }
}

pub struct DataloaderIterator<'a> {
    records: &'a [GameRecord],
    batch_size: usize,
    current_index: usize,
}

impl<'a> Iterator for DataloaderIterator<'a> {
    type Item = &'a [GameRecord];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.records.len() {
            return None;
        }

        let end = std::cmp::min(self.current_index + self.batch_size, self.records.len());
        let batch = &self.records[self.current_index..end];
        self.current_index = end;
        Some(batch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() -> ResultBoxErr<()> {
        let cwd = std::env::current_dir().unwrap();
        println!("Current working directory: {:?}", cwd);

        let new_dir = std::path::Path::new("reversi");
        if let Err(e) = std::env::set_current_dir(new_dir) {
            eprintln!("Failed to change directory: {}", e);
        }

        Ok(())
    }
}
