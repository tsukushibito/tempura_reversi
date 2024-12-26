use crate::Model;

pub trait Optimizer {
    fn step(&mut self, params: &mut [f32], grads: &[f32]);
    fn set_learning_rate(&mut self, lr: f32);
    fn get_learning_rate(&self) -> f32;
    fn reset(&mut self);
}

pub trait LRScheduler {
    fn step(&mut self, optimizer: &mut dyn Optimizer);
}
pub struct Dataloader;
pub struct LossFunction;

pub struct Learner<O, L>
where
    O: Optimizer,
    L: LRScheduler,
{
    model: Model,

    optimizer: O,
    lr_scheduler: Option<L>,
    train_dataloader: Dataloader,
    valid_dataloader: Option<Dataloader>,

    num_epochs: usize,
    batch_size: usize,
    seed: Option<u64>,

    loss_function: LossFunction,

    current_epoch: usize,
    best_loss: f32,
}
