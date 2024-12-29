use crate::{
    dataloader::Dataloader, loss_function::LossFunction, lr_scheduler::LRScheduler, model::Model,
    optimizer::Optimizer,
};

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
    batch_size: usize,
    seed: Option<u64>,

    loss_function: L,

    current_epoch: usize,
    best_loss: f32,
}
