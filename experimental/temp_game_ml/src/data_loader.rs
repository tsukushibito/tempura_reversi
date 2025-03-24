use crate::Dataset;

pub struct DataLoader<D: Dataset> {
    dataset: D,
    batch_size: usize,
    index: usize,
}

impl<D: Dataset> DataLoader<D> {
    pub fn new(dataset: D, batch_size: usize) -> Self {
        DataLoader {
            dataset,
            batch_size,
            index: 0,
        }
    }
}

impl<D: Dataset> Iterator for DataLoader<D> {
    type Item = Vec<D::Sample>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.dataset.len() {
            return None;
        }
        let end = (self.index + self.batch_size).min(self.dataset.len());
        let batch = (self.index..end)
            .map(|i| self.dataset.get_item(i))
            .collect();
        self.index = end;
        Some(batch)
    }
}
