use derive_builder::Builder;
use indicatif::ProgressBar;
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};
use serde::{Deserialize, Serialize};

use crate::{ResultBoxErr, SparseVector};

use super::{
    dataloader::Dataloader, get_data_items_from_record, loss_function::LossFunction,
    lr_scheduler::LrScheduler, optimizer::Optimizer, transpose, DataItem, GameRecord, Model,
    ModelInput,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarlyStoppingConfig {
    pub patience: usize,
    pub min_delta: f32,
}

#[derive(Debug, Builder)]
pub struct Learner<O, S, L>
where
    O: Optimizer,
    S: LrScheduler,
    L: LossFunction,
{
    pub model: Model,
    train_dataloader: Dataloader,

    optimizer: O,
    num_epochs: usize,
    loss_function: L,

    #[builder(default = "None")]
    lr_scheduler: Option<S>,

    #[builder(default = "None")]
    valid_dataloader: Option<Dataloader>,

    #[builder(default = "None")]
    early_stopping: Option<EarlyStoppingConfig>,

    #[builder(default, setter(skip))]
    best_loss: f32,

    #[builder(default, setter(skip))]
    patience_counter: usize,

    #[builder(default, setter(skip))]
    pub last_loss: f32,
}

impl<O, S, L> Learner<O, S, L>
where
    O: Optimizer,
    S: LrScheduler,
    L: LossFunction,
{
    pub fn fit(&mut self, progress_bar: &ProgressBar) -> ResultBoxErr<()> {
        self.best_loss = f32::MAX;

        for _epoch in 0..self.num_epochs {
            self.train_dataloader.reset()?;

            let mut losses = Vec::new();
            for batch in self.train_dataloader.iter_batches() {
                let loss = train_single_batch(
                    &mut self.model,
                    &mut self.optimizer,
                    &self.loss_function,
                    batch,
                );
                losses.push(loss);
            }

            let sum: f32 = losses.iter().sum();
            let loss_avarage = sum / losses.len() as f32;
            let dif = loss_avarage - self.last_loss;
            progress_bar.set_message(format!("Loss:{loss_avarage:0.2}({dif:0.2})"));
            self.last_loss = loss_avarage;

            if let Some(valid_loader) = &self.valid_dataloader {
                let validation_loss = self.evaluate(valid_loader)?;

                if let Some(early_stop_config) = &self.early_stopping {
                    if validation_loss + early_stop_config.min_delta < self.best_loss {
                        self.best_loss = validation_loss;
                        self.patience_counter = 0;
                    } else {
                        self.patience_counter += 1;
                        if self.patience_counter >= early_stop_config.patience {
                            return Ok(());
                        }
                    }
                }
            }

            // if let Some(lr_scheduler) = &mut self.lr_scheduler {
            //     lr_scheduler.step(&mut self.optimizer);
            // }

            progress_bar.inc(1);
        }

        progress_bar.finish();

        Ok(())
    }

    pub fn evaluate(&self, dataloader: &Dataloader) -> ResultBoxErr<f32> {
        let mut total_loss = 0.0;
        let mut count = 0.0;

        for batch in dataloader.iter_batches() {
            let features: Vec<SparseVector> =
                batch.iter().map(|item| item.feature.clone()).collect();
            let targets: Vec<f32> = batch.iter().map(|item| item.target).collect();
            let predictions = self.model.forward(&features);
            let loss = self.loss_function.compute(&predictions, targets.as_slice());

            total_loss += loss.value * predictions.len() as f32;
            count += predictions.len() as f32;
        }

        Ok(total_loss / count)
    }
}

fn compute_gradients(grad_outputs: &[f32], features: &[SparseVector]) -> SparseVector {
    let mut grad_weights = grad_outputs
        .iter()
        .zip(features.iter())
        .map(|(&grad_output, feature)| feature.clone() * grad_output)
        .reduce(|g1, g2| g1 + g2)
        .unwrap();

    grad_weights = grad_weights / grad_outputs.len() as f32;

    grad_weights
}

fn train_single_batch<O, L>(
    model: &mut Model,
    optimizer: &mut O,
    loss_function: &L,
    records: &[GameRecord],
) -> f32
where
    O: Optimizer,
    L: LossFunction,
{
    let items_by_record: Vec<Vec<DataItem>> = records
        .par_iter()
        .map(|record| get_data_items_from_record(record))
        .collect();
    let items_by_phase = transpose(items_by_record);

    items_by_phase
        .into_iter()
        .enumerate()
        .for_each(|(phase, items)| {
            let (features, targets): (Vec<SparseVector>, Vec<f32>) =
                items.into_iter().map(|i| (i.feature, i.target)).unzip();

            let inputs: Vec<ModelInput> = features
                .iter()
                .map(|f| ModelInput {
                    phase,
                    feature: f.clone(),
                })
                .collect();
            let predictions: Vec<f32> = model.forward(&inputs);
            let loss = loss_function.compute(&predictions, &targets);
            let grads = compute_gradients(&loss.grad, &features);
            optimizer.step(&mut model.params, &grads);
        });

    model
        .params
        .into_par_iter()
        .zip(items_by_phase)
        .map(|(param, items)| {});

    let features: Vec<SparseVector> = datas.iter().map(|d| d.feature.clone()).collect();
    let targets: Vec<f32> = datas.iter().map(|d| d.target).collect();

    let predictions: Vec<f32> = model.forward(&features);
    let loss = loss_function.compute(&predictions, &targets);
    let grads = compute_gradients(&loss.grad, &features);
    optimizer.step(&mut model.params, &grads);

    loss.value
}

#[cfg(test)]
mod tests {
    use crate::{
        ml::{Adam, Mse},
        TempuraEvaluator,
    };

    use super::*;
}
