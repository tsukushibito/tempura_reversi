use std::fs::File;
use std::io::Write;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{ResultBoxErr, SparseVector};

use super::{
    dataloader::Dataloader, loss_function::LossFunction, lr_scheduler::LrScheduler,
    optimizer::Optimizer, Model,
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
    current_epoch: usize,

    #[builder(default, setter(skip))]
    best_loss: f32,

    #[builder(default, setter(skip))]
    patience_counter: usize,
}

impl<O, S, L> Learner<O, S, L>
where
    O: Optimizer,
    S: LrScheduler,
    L: LossFunction,
{
    pub fn fit(&mut self) -> ResultBoxErr<()> {
        self.best_loss = f32::MAX;

        for epoch in 0..self.num_epochs {
            println!("Epoch {}", epoch + 1);
            self.train_dataloader.reset();

            let mut copied: Vec<Model> = Default::default();
            copied.push(self.model.clone());
            for batch in self.train_dataloader.iter_batches() {
                let inputs: Vec<SparseVector> =
                    batch.iter().map(|item| item.input.clone()).collect();
                let targets: Vec<f32> = batch.iter().map(|item| item.target).collect();

                let loss = train_single_batch(
                    &mut self.model,
                    &mut self.optimizer,
                    &self.loss_function,
                    &inputs,
                    &targets,
                );

                println!("Loss: {:.4}", loss);
                copied.push(self.model.clone());
            }

            // save_all_weights_to_csv(&copied, "params.csv")?;

            if let Some(valid_loader) = &self.valid_dataloader {
                let validation_loss = self.evaluate(valid_loader)?;
                println!("Validation Loss: {:.4}", validation_loss);

                if let Some(early_stop_config) = &self.early_stopping {
                    if validation_loss + early_stop_config.min_delta < self.best_loss {
                        self.best_loss = validation_loss;
                        self.patience_counter = 0;
                    } else {
                        self.patience_counter += 1;
                        println!(
                            "Early Stopping Check: Patience {}/{}",
                            self.patience_counter, early_stop_config.patience
                        );
                        if self.patience_counter >= early_stop_config.patience {
                            println!("Early stopping triggered at epoch {}.", self.current_epoch);
                            return Ok(());
                        }
                    }
                }
            }

            if let Some(lr_scheduler) = &mut self.lr_scheduler {
                lr_scheduler.step(&mut self.optimizer);
            }

            println!("Epoch {} completed.\n", epoch + 1);
        }

        Ok(())
    }

    pub fn evaluate(&self, dataloader: &Dataloader) -> ResultBoxErr<f32> {
        let mut total_loss = 0.0;
        let mut count = 0.0;

        for batch in dataloader.iter_batches() {
            let inputs: Vec<SparseVector> = batch.iter().map(|item| item.input.clone()).collect();
            let targets: Vec<f32> = batch.iter().map(|item| item.target).collect();

            let predictions = self.model.forward(&inputs);
            let loss = self.loss_function.compute(&predictions, targets.as_slice());

            total_loss += loss.value * predictions.len() as f32;
            count += predictions.len() as f32;
        }

        Ok(total_loss / count)
    }
}

fn compute_gradients(grad_outputs: &[f32], inputs: &[SparseVector]) -> SparseVector {
    let mut grad_weights = grad_outputs
        .iter()
        .zip(inputs.iter())
        .map(|(&grad_output, input)| input.clone() * grad_output)
        .reduce(|g1, g2| g1 + g2)
        .unwrap();

    grad_weights = grad_weights / grad_outputs.len() as f32;

    grad_weights
}

fn train_single_batch<O, L>(
    model: &mut Model,
    optimizer: &mut O,
    loss_function: &L,
    inputs: &[SparseVector],
    targets: &[f32],
) -> f32
where
    O: Optimizer,
    L: LossFunction,
{
    // モデルの予測
    let predictions = model.forward(inputs);

    // 損失の計算
    let loss = loss_function.compute(&predictions, targets);

    // 勾配の計算
    let grad_outputs = &loss.grad;
    let grads = compute_gradients(grad_outputs, inputs);

    // パラメータの更新
    optimizer.step(&mut model.weights, &grads);

    // 損失値を返す
    loss.value
}

fn save_all_weights_to_csv(models: &[Model], file_name: &str) -> std::io::Result<()> {
    let mut file = File::create(file_name)?;

    for model in models {
        for (i, weight) in model.weights.iter().enumerate() {
            if i > 0 {
                write!(file, ",")?;
            }
            write!(file, "{}", weight)?;
        }
        writeln!(file, ",")?; // 最後にカンマを追加して改行
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        ml::{Adam, Mse},
        TempuraEvaluator,
    };

    use super::*;

    #[test]
    fn test_train_single_batch() {
        let mut model = Model::new(3);
        let mut optimizer = Adam::new(0.01, 0.9, 0.999, 1e-8);
        let loss_function = Mse::new();

        let inputs = vec![
            SparseVector::new(vec![0, 1], vec![1.0, 2.0], 3).unwrap(),
            SparseVector::new(vec![1, 2], vec![3.0, 4.0], 3).unwrap(),
        ];
        let targets = vec![5.0, 18.0];

        let initial_weights = model.weights.clone();

        let loss = train_single_batch(
            &mut model,
            &mut optimizer,
            &loss_function,
            &inputs,
            &targets,
        );

        // 損失が正しい範囲内かを確認
        assert!(loss > 0.0);

        // モデルのパラメータが更新されていることを確認
        assert_ne!(model.weights, initial_weights);
    }

    #[test]
    fn test_loss_decrease() {
        let mut model = Model::new(3);
        let mut optimizer = Adam::new(0.01, 0.9, 0.999, 1e-8);
        let loss_function = Mse::new();

        let inputs = vec![
            SparseVector::new(vec![0, 1], vec![1.0, 2.0], 3).unwrap(),
            SparseVector::new(vec![1, 2], vec![3.0, 4.0], 3).unwrap(),
        ];
        let targets = vec![5.0, 18.0];

        let mut previous_loss = f32::MAX;

        for _ in 0..100 {
            let loss = train_single_batch(
                &mut model,
                &mut optimizer,
                &loss_function,
                &inputs,
                &targets,
            );
            println!("Loss: {:.6}", loss);

            // 損失が減少していることを確認
            assert!(loss <= previous_loss, "Loss did not decrease!");
            previous_loss = loss;
        }
    }

    #[test]
    fn test_train_model() {
        let evaluator = TempuraEvaluator::default();
        let input_size = evaluator.feature_size();
        let mut model = evaluator.model;

        let mut optimizer = Adam::new(0.01, 0.9, 0.999, 1e-8);
        let loss_function = Mse::new();

        let inputs = vec![
            SparseVector::new(vec![0, 1], vec![1.0, 2.0], input_size).unwrap(),
            SparseVector::new(vec![1, 2], vec![3.0, 4.0], input_size).unwrap(),
        ];
        let targets = vec![5.0, 18.0];

        let mut previous_loss = f32::MAX;

        for _ in 0..100 {
            let loss = train_single_batch(
                &mut model,
                &mut optimizer,
                &loss_function,
                &inputs,
                &targets,
            );
            println!("Loss: {:.6}", loss);

            // 損失が減少していることを確認
            assert!(loss <= previous_loss, "Loss did not decrease!");
            previous_loss = loss;
        }
    }
}
