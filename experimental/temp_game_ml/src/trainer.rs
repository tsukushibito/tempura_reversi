use crate::{DataLoader, Dataset, LossFunction, Model, Optimizer};

pub struct Trainer<M, O, L, D>
where
    M: Model,
    O: Optimizer,
    L: LossFunction,
    D: Dataset,
{
    pub model: M,
    pub optimizer: O,
    pub loss_function: L,
    pub dataloader: DataLoader<D>,
}

impl<M, O, L, D> Trainer<M, O, L, D>
where
    M: Model,
    O: Optimizer,
    L: LossFunction,
    D: Dataset<Sample = (Vec<f32>, Vec<f32>)>,
{
    pub fn train_epoch(&mut self) {
        for batch in &mut self.dataloader {
            for (input, target) in batch {
                let predictions = self.model.forward(&input);
                let loss = self.loss_function.compute_loss(&predictions, &target);
                let grad_output = self.loss_function.compute_gradient(&predictions, &target);
                let (weight_grads, bias_grads) = self.model.backword(&input, &grad_output);
                self.optimizer
                    .update(&mut self.model, &weight_grads, &bias_grads);
            }
        }
    }
}
