use std::{fs::File, io::Read, path::Path, rc::Rc};

use indicatif::ProgressBar;
use rand::{seq::SliceRandom, thread_rng};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::ResultBoxErr;

use super::{get_data_items_from_record, DataItem, GameRecord};

#[derive(Debug, Clone)]
pub struct Dataloader {
    records_file_path: String,
    items: Rc<Vec<DataItem>>,
    batch_size: usize,
    current_index: usize,
}

impl Dataloader {
    pub fn new<P: AsRef<Path>>(
        records_file_path: P,
        batch_size: usize,
        shuffle: bool,
    ) -> ResultBoxErr<Self> {
        println!(
            "[Dataloader::new()] records_file_path={:?}",
            records_file_path.as_ref().to_str().unwrap()
        );
        let mut file = File::open(&records_file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let records: Vec<GameRecord> = bincode::deserialize(&buffer)?;

        println!("Converting game records to data items...");
        let pb = ProgressBar::new(records.len() as u64);
        let mut items: Vec<DataItem> = records
            .par_iter()
            .flat_map(|record| {
                let items = get_data_items_from_record(record);
                pb.inc(1);
                items
            })
            .collect();

        if shuffle {
            let mut rng = thread_rng();
            items.shuffle(&mut rng);
        }

        let records_file_path: String = records_file_path
            .as_ref()
            .to_str()
            .ok_or("invalid path")?
            .to_string();

        Ok(Dataloader {
            records_file_path,
            items: Rc::new(items),
            batch_size,
            current_index: 0,
        })
    }

    pub fn next_batch(&mut self) -> Option<&[DataItem]> {
        if self.current_index >= self.items.len() {
            return None;
        }

        let end = std::cmp::min(self.current_index + self.batch_size, self.items.len());
        let batch = &self.items[self.current_index..end];
        self.current_index = end;
        Some(batch)
    }

    pub fn reset(&mut self) {
        self.current_index = 0;
    }

    pub fn iter_batches(&self) -> DataloaderIterator {
        DataloaderIterator {
            items: &self.items,
            batch_size: self.batch_size,
            current_index: 0,
        }
    }
}

pub struct DataloaderIterator<'a> {
    items: &'a [DataItem],
    batch_size: usize,
    current_index: usize,
}

impl<'a> Iterator for DataloaderIterator<'a> {
    type Item = &'a [DataItem];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.items.len() {
            return None;
        }

        let end = std::cmp::min(self.current_index + self.batch_size, self.items.len());
        let batch = &self.items[self.current_index..end];
        self.current_index = end;
        Some(batch)
    }
}
