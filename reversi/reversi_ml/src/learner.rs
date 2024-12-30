use crate::{
    dataloader::Dataloader, loss_function::LossFunction, lr_scheduler::LRScheduler, model::Model,
    optimizer::Optimizer, sparse_vector::SparseVector, DynResult,
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

impl<O, S, L> Learner<O, S, L> {
    pub fn fit(&mut self) -> DynResult<()> {
        for epoch in 0..self.num_epochs {
            println!("Epoch {}", epoch + 1);
            self.train_dataloader.reset();

            for batch in self.train_dataloader.iter_batches() {
                // let (inputs, targets) = batch;
                batch.iter().map(|item| {item.} )

                // フォワードパス
                let predictions = self.model.forward(inputs);

                // 損失の計算
                let loss = self.loss_function.compute(&predictions, targets.as_slice());

                // バックワードパス（勾配の計算）
                let grad_output = Array1::from(loss.grad.clone());
                let grads = self.model.backward(&grad_output, inputs_matrix);

                // パラメータの更新
                self.optimizer.step(&mut self.model.weights, &grads.weights);
                self.optimizer.step(&mut [self.model.bias], &[grads.bias]);

                // 損失の出力
                println!("Loss: {:.4}", loss.value);
            }

            // 学習率スケジューラのステップ
            if let Some(lr_scheduler) = &mut self.lr_scheduler {
                lr_scheduler.step(&mut *self.optimizer);
            }

            println!("Epoch {} completed.\n", epoch + 1);
        }

        Ok(())
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
