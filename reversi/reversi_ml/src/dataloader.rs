use std::{fs::File, io::Read};

use rand::{seq::SliceRandom, thread_rng};
use reversi::{BitBoard, DynResult, Game, GameRecord, Position};

use crate::sparse_vector::SparseVector;

#[derive(Debug, Clone, Default)]
pub struct Item {
    input: SparseVector,
    target: f32,
}

#[derive(Debug)]
pub struct Dataloader {
    items: Vec<Item>,
    batch_size: usize,
    shuffle: bool,
    current_index: usize,
}

impl Dataloader {
    pub fn new(file_path: &str, batch_size: usize, shuffle: bool) -> DynResult<Self> {
        let mut file = File::open(file_path)?;
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        let records: Vec<GameRecord> = bincode::deserialize(&buf)?;
        let mut items = make_items_from_game_records(&records);

        if shuffle {
            let mut rng = thread_rng();
            items.shuffle(&mut rng);
        }

        Ok(Dataloader {
            items,
            batch_size,
            shuffle,
            current_index: 0,
        })
    }

    pub fn next_batch(&mut self) -> Option<&[Item]> {
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
        if self.shuffle {
            let mut rng = thread_rng();
            self.items.shuffle(&mut rng);
        }
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
    items: &'a [Item],
    batch_size: usize,
    current_index: usize,
}

impl<'a> Iterator for DataloaderIterator<'a> {
    type Item = &'a [Item];

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

fn make_items_from_game_records(records: &[GameRecord]) -> Vec<Item> {
    let model = reversi::Model::default();

    let mut items = Vec::new();

    for record in records {
        let diff = record.black_score as i32 - record.white_score as i32;
        let target = diff as f32;

        let mut game = Game::initial();

        for i in 0..=record.moves.len() {
            let board = BitBoard::from_board(game.board());

            let feature = model.feature(&board);

            items.push(Item {
                input: SparseVector::new(feature.indices, feature.values, feature.length).unwrap(),
                target,
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
