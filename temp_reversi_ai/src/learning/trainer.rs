use std::sync::Arc;

use rayon::prelude::*;

use super::{loss_function::LossFunction, optimizer::Optimizer, Dataset, GameDataset, Model};
use crate::utils::SparseVector;

/// Trainer responsible for managing epochs, batches, and model updates
pub struct Trainer<L: LossFunction, O: Optimizer> {
    model: Model,
    loss_fn: L,
    optimizers: Vec<O>,
    batch_size: usize,
    epochs: usize,

    // Made public to allow access from training_pipeline.rs
    pub validation_overall_losses: Vec<f32>,
    pub validation_phase_losses: Vec<Vec<(usize, f32)>>, // (phase, avg_loss)
}

impl<L: LossFunction, O: Optimizer + Send + Sync + Clone> Trainer<L, O> {
    /// Creates a new Trainer with specified parameters and an optional regularizer.
    pub fn new(
        feature_size: usize,
        loss_fn: L,
        optimizer: O,
        batch_size: usize,
        epochs: usize,
        model_path: Option<&str>,
    ) -> Self {
        let optimizers = vec![optimizer.clone(); 60];
        let model = if let Some(path) = model_path {
            Model::load(path).expect("Failed to load model.")
        } else {
            Model {
                weights: vec![vec![0.0; feature_size]; 60],
                bias: 0.0,
            }
        };

        Self {
            model,
            loss_fn,
            optimizers,
            batch_size,
            epochs,
            validation_overall_losses: Vec::new(),
            validation_phase_losses: Vec::new(),
        }
    }

    /// Returns a reference to the trained model
    pub fn model(&self) -> &Model {
        &self.model
    }

    /// Trains the model on the training dataset and evaluates it on the validation dataset after each epoch.
    pub fn train(
        &mut self,
        train_dataset: &mut GameDataset,
        validation_dataset: &GameDataset,
        reporter: Option<Arc<dyn crate::utils::ProgressReporter + Send + Sync>>,
    ) {
        if let Some(r) = &reporter {
            r.on_start(self.epochs);
        }
        // Pre-expand validation data once
        let validation_data = validation_dataset.extract_all_training_data();

        for epoch in 0..self.epochs {
            // println!("🚀 Starting Epoch {}/{}", epoch + 1, self.epochs);
            let start_time = std::time::Instant::now();

            train_dataset.shuffle();

            let batches = train_dataset.extract_training_data_in_batches(self.batch_size);

            for (_batch_idx, batch) in batches.enumerate() {
                self.train_batch(&batch);
                // println!("Batch {} completed.", _batch_idx + 1);
            }
            let duration = start_time.elapsed();
            // println!(
            //     "✅ Epoch {}/{} completed. {:?}",
            //     epoch + 1,
            //     self.epochs,
            //     duration
            // );

            let (overall_loss, phase_losses) = self.validate(&validation_data);
            self.validation_overall_losses.push(overall_loss);
            self.validation_phase_losses.push(phase_losses);

            if let Some(r) = &reporter {
                r.on_progress(
                    epoch + 1,
                    self.epochs,
                    Some(&format!("duration: {duration:?}")),
                );
            }
        }
        if let Some(r) = &reporter {
            r.on_complete();
        }
    }

    fn train_batch(&mut self, batch: &Dataset) {
        let predictions = self.model.predict(&batch.features);
        let phases: Vec<usize> = batch.features.iter().map(|f| f.phase).collect();
        let (losses, phase_losses) =
            self.loss_fn
                .compute_loss_by_phase(&predictions, &batch.labels, &phases);
        let gradients = self.loss_fn.compute_gradient(&predictions, &batch.labels);

        let num_phases = self.model.weights.len();
        let mut phase_sparse_grads: Vec<Vec<SparseVector>> = vec![Vec::new(); num_phases];
        for (feature, &grad) in batch.features.iter().zip(gradients.iter()) {
            let sparse_grad = SparseVector::new(
                feature.vector.indices().to_vec(),
                feature.vector.values().iter().map(|&v| grad * v).collect(),
                feature.vector.size(),
            )
            .unwrap();

            phase_sparse_grads[feature.phase].push(sparse_grad);
        }

        let num_threads = rayon::current_num_threads();
        let chunk_size = (self.model.weights.len() + num_threads - 1) / num_threads;

        self.model
            .weights
            .par_chunks_mut(chunk_size)
            .zip(self.optimizers.par_chunks_mut(chunk_size))
            .enumerate()
            .for_each(|(chunk_index, (weights_chunk, optimizers_chunk))| {
                for (i, (weight, optimizer)) in weights_chunk
                    .iter_mut()
                    .zip(optimizers_chunk.iter_mut())
                    .enumerate()
                {
                    let phase = chunk_index * chunk_size + i;
                    let mut dummy_bias = 0.0;
                    for sparse_grad in &phase_sparse_grads[phase] {
                        optimizer.update(weight, &mut dummy_bias, sparse_grad, 0.0);
                    }
                }
            });

        let _overall_avg_loss: f32 = losses.iter().sum::<f32>() / losses.len() as f32;
        let _phase_loss_line: String = phase_losses
            .iter()
            .enumerate()
            .filter_map(|(phase, losses_vec)| {
                if !losses_vec.is_empty() {
                    let avg = losses_vec.iter().sum::<f32>() / losses_vec.len() as f32;
                    Some(format!("Phase {}: {:.6}", phase, avg))
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
            .join(", ");
    }

    /// Validates the model on the provided pre-expanded Dataset and prints
    /// the overall average loss as well as the per-phase average losses.
    pub fn validate(&self, validation_data: &Dataset) -> (f32, Vec<(usize, f32)>) {
        let all_features = &validation_data.features;
        let all_labels = &validation_data.labels;
        let predictions = self.model.predict(all_features);
        let phases: Vec<usize> = all_features.iter().map(|f| f.phase).collect();

        let (losses, phase_losses) =
            self.loss_fn
                .compute_loss_by_phase(&predictions, all_labels, &phases);

        let overall_avg_loss = if !losses.is_empty() {
            losses.iter().sum::<f32>() / losses.len() as f32
        } else {
            0.0
        };

        let mut phase_loss_result: Vec<(usize, f32)> = phase_losses
            .iter()
            .enumerate()
            .filter_map(|(phase, losses_vec)| {
                if !losses_vec.is_empty() {
                    let avg = losses_vec.iter().sum::<f32>() / losses_vec.len() as f32;
                    Some((phase, avg))
                } else {
                    None
                }
            })
            .collect();

        phase_loss_result.sort_by_key(|&(phase, _)| phase);

        // for (phase, avg_loss) in &phase_loss_result {
        //     print!("Phase {}: {:.6}, ", phase, avg_loss);
        // }
        // println!();

        (overall_avg_loss, phase_loss_result)
    }
}
