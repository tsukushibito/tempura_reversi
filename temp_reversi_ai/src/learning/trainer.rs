use super::{loss_function::LossFunction, optimizer::Optimizer, Dataset, GameDataset, Model};
use crate::utils::SparseVector;

/// Trainer responsible for managing epochs, batches, and model updates
pub struct Trainer<L: LossFunction, O: Optimizer> {
    model: Model,
    loss_fn: L,
    optimizer: O,
    batch_size: usize,
    epochs: usize,
}

impl<L: LossFunction, O: Optimizer> Trainer<L, O> {
    /// Creates a new Trainer with specified parameters
    pub fn new(
        feature_size: usize,
        loss_fn: L,
        optimizer: O,
        batch_size: usize,
        epochs: usize,
    ) -> Self {
        Self {
            model: Model {
                weights: vec![vec![0.0; feature_size]; 60],
                bias: 0.0,
            },
            loss_fn,
            optimizer,
            batch_size,
            epochs,
        }
    }

    /// Returns a reference to the trained model
    pub fn model(&self) -> &Model {
        &self.model
    }

    /// Trains the model using GameDataset with batch processing
    pub fn train(&mut self, game_dataset: &mut GameDataset) {
        for epoch in 0..self.epochs {
            println!("🚀 Starting Epoch {}/{}", epoch + 1, self.epochs);

            game_dataset.shuffle();

            let batches = game_dataset.extract_training_data_in_batches(self.batch_size);

            for (_batch_idx, batch) in batches.enumerate() {
                self.train_batch(&batch);
                // println!("Batch {} completed.", _batch_idx + 1);
            }

            println!("✅ Epoch {}/{} completed.", epoch + 1, self.epochs);
        }
    }

    /// Trains the model on a single batch
    fn train_batch(&mut self, batch: &Dataset) {
        let predictions = self.model.predict(&batch.features);
        let losses = self.loss_fn.compute_loss(&predictions, &batch.labels);
        let gradients = self.loss_fn.compute_gradient(&predictions, &batch.labels);

        batch
            .features
            .iter()
            .zip(gradients.iter())
            .for_each(|(features, &grad)| {
                let sparse_grad = SparseVector::new(
                    features.vector.indices().to_vec(),
                    features.vector.values().iter().map(|&v| grad * v).collect(),
                    features.vector.size(),
                )
                .unwrap();

                self.optimizer.update(
                    &mut self.model.weights[features.phase],
                    &mut self.model.bias,
                    &sparse_grad,
                    0.0,
                );
            });

        let avg_loss: f32 = losses.iter().sum::<f32>() / losses.len() as f32;
        println!("Loss: {:.6}", avg_loss);
    }
}
