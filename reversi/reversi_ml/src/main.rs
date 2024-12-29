pub mod dataloader;
pub mod learner;
pub mod loss_function;
pub mod lr_scheduler;
pub mod model;
pub mod optimizer;
pub mod sparse_vector;

pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() {
    println!("Hello, world!");
}
