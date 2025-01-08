use crate::{ResultBoxErr, SparseVector};

use super::{
    dataloader::Dataloader, loss_function::LossFunction, lr_scheduler::LRScheduler,
    optimizer::Optimizer, Model,
};

#[derive(Debug, Clone)]
pub struct EarlyStoppingConfig {
    pub patience: usize,
    pub min_delta: f32,
}

#[derive(Debug)]
pub struct Learner<O, S, L>
where
    O: Optimizer,
    S: LRScheduler,
    L: LossFunction,
{
    model: Model,

    optimizer: O,
    lr_scheduler: Option<S>,
    train_dataloader: Dataloader,
    valid_dataloader: Option<Dataloader>,

    num_epochs: usize,

    loss_function: L,

    current_epoch: usize,
    best_loss: f32,

    early_stopping: Option<EarlyStoppingConfig>,
    patience_counter: usize,
}

impl<O, S, L> Learner<O, S, L>
where
    O: Optimizer,
    S: LRScheduler,
    L: LossFunction,
{
    pub fn fit(&mut self) -> ResultBoxErr<()> {
        for epoch in 0..self.num_epochs {
            println!("Epoch {}", epoch + 1);
            self.train_dataloader.reset();

            for batch in self.train_dataloader.iter_batches() {
                let inputs: Vec<SparseVector> =
                    batch.iter().map(|item| item.input.clone()).collect();
                let targets: Vec<f32> = batch.iter().map(|item| item.target).collect();

                let predictions = self.model.forward(&inputs);

                let loss = self.loss_function.compute(&predictions, targets.as_slice());

                let grad_outputs = loss.grad;
                let grads = compute_gradients(&grad_outputs, &inputs);

                self.optimizer.step(&mut self.model.weights, &grads);

                println!("Loss: {:.4}", loss.value);
            }

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
