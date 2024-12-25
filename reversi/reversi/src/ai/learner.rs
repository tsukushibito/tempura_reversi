use crate::Model;

pub struct Optimizer;
pub struct LRScheduler;
pub struct Dataloader;
pub struct LossFunction;

pub struct Learner {
    model: Model,

    optimizer: Optimizer,
    lr_scheduler: Option<LRScheduler>,
    train_dataloader: Dataloader,
    valid_dataloader: Option<Dataloader>,

    num_epochs: usize,
    batch_size: usize,
    seed: Option<u64>,

    loss_function: LossFunction,

    current_epoch: usize,
    best_loss: f32,
}
