use burn::{
    data::{
        dataloader::batcher::Batcher,
        dataset::{Dataset, HuggingfaceDatasetLoader, InMemDataset},
    },
    prelude::Backend,
    tensor::Tensor,
};
use csv;
use reversi::{BitBoard, Game, GameRecord};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ReversiItem {
    pub feature_size: usize,
    pub feature: Vec<f32>,
    pub value: f32,
}

pub struct ReversiDataset {
    dataset: InMemDataset<ReversiItem>,
}

impl Dataset<ReversiItem> for ReversiDataset {
    fn get(&self, index: usize) -> Option<ReversiItem> {
        self.dataset.get(index)
    }

    fn len(&self) -> usize {
        self.dataset.len()
    }
}

impl ReversiDataset {
    pub fn train() -> Option<Self> {
        Self::new("train")
    }

    pub fn validation() -> Option<Self> {
        Self::new("validation")
    }

    pub fn test() -> Option<Self> {
        Self::new("test")
    }

    pub fn new(csv_name: &str) -> Option<Self> {
        let mut rdr = csv::ReaderBuilder::new();
        let rdr = rdr.delimiter(b'\t');
        let dataset: InMemDataset<ReversiItem> = InMemDataset::from_csv(csv_name, rdr).ok()?;
        Some(Self { dataset })
    }

    pub fn d_input(&self) -> Option<usize> {
        let item = self.dataset.get(0)?;
        Some(item.feature_size)
    }
}

#[derive(Clone, Debug)]
pub struct ReversiBatcher<B: Backend> {
    device: B::Device,
}

impl<B: Backend> ReversiBatcher<B> {
    pub fn new(device: B::Device) -> Self {
        Self { device }
    }
}

#[derive(Clone, Debug)]
pub struct ReversiBatch<B: Backend> {
    pub inputs: Tensor<B, 2>,
    pub targets: Tensor<B, 1>,
}

impl<B: Backend> Batcher<ReversiItem, ReversiBatch<B>> for ReversiBatcher<B> {
    fn batch(&self, items: Vec<ReversiItem>) -> ReversiBatch<B> {
        let inputs = items
            .iter()
            .map(|item| {
                Tensor::<B, 1>::from_floats(item.feature.as_slice(), &self.device).unsqueeze()
            })
            .collect::<Vec<_>>();
        let inputs = Tensor::cat(inputs, 0).to_device(&self.device);

        let targets = items
            .iter()
            .map(|item| Tensor::<B, 1>::from_floats([item.value], &self.device))
            .collect::<Vec<_>>();
        let targets = Tensor::cat(targets, 0).to_device(&self.device);

        ReversiBatch { inputs, targets }
    }
}
