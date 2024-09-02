use std::{path::Path, sync::Arc};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};

use crate::{
    ml::{Adam, Dataloader, LearnerBuilder, Model, Mse, Sgd, StepLr},
    Config, ResultBoxErr, TempuraEvaluator,
};

pub fn training(config: &str) -> ResultBoxErr<()> {
    println!("config: {}", config);
    let config = Config::from_file(config)?;

    let models_file = config.training_models_path();
    let models = if !Path::exists(&models_file) {
        let evaluator = TempuraEvaluator::default();
        let input_size = evaluator.feature_size();
        vec![Model::new(input_size); 60]
    } else {
        Model::load_models(&models_file)?
    };

    println!("base_path: {}", config.base_path);

    let data_loaders = Dataloader::from_data_file(
        config.training_train_data_file_path(),
        config.training.batch_size,
    )?;

    let valid_data_loaders = Dataloader::from_data_file(
        config.training_valid_data_file_path(),
        config.training.batch_size,
    )?;

    let multi_progress = Arc::new(MultiProgress::new());
    let style = ProgressStyle::with_template(
        "[{elapsed_precise}][{prefix}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");

    let temp: Vec<(usize, Model, Dataloader, Dataloader)> = models
        .into_iter()
        .zip(data_loaders)
        .zip(valid_data_loaders)
        .enumerate()
        .map(|(phase, ((model, data_loader), valid_loader))| {
            (phase, model, data_loader, valid_loader)
        })
        .rev()
        .collect();

    let models_and_losses: Vec<(Model, f32)> = temp
        .into_par_iter()
        .map(|(phase, model, data_loader, valid_loader)| {
            let multi_progress = Arc::clone(&multi_progress);

            let progress_bar = multi_progress.add(ProgressBar::new(config.training.epochs as u64));
            progress_bar.set_style(style.clone());
            progress_bar.set_prefix(format!("{phase:02}"));
            let optimizer = Adam::new(0.001, 0.9, 0.999, 1e-8);
            // let optimizer = Sgd::new(0.001);
            let loss_function = Mse::new();
            let lr_scheduler = StepLr::new(50, 0.1);

            let mut learner = LearnerBuilder::default()
                .model(model.clone())
                .train_dataloader(data_loader)
                .valid_dataloader(Some(valid_loader))
                .optimizer(optimizer)
                .num_epochs(config.training.epochs)
                .loss_function(loss_function)
                .lr_scheduler(Some(lr_scheduler))
                .build()
                .unwrap();

            learner.fit(&progress_bar).unwrap();

            (learner.model, learner.last_loss)
        })
        .collect();

    multi_progress.clear()?;

    let models = models_and_losses
        .iter()
        .map(|elem| elem.0.clone())
        .collect();

    let losses: Vec<f32> = models_and_losses.iter().map(|elem| elem.1).collect();
    let sum: f32 = losses.iter().sum();
    let loss_avarage = sum / losses.len() as f32;
    println!("loss_avarage: {loss_avarage:?}");

    Model::save_models(&models, models_file)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_training() -> ResultBoxErr<()> {
        let cwd = std::env::current_dir().unwrap();
        println!("Current working directory: {:?}", cwd);

        let new_dir = std::path::Path::new("reversi");
        if let Err(e) = std::env::set_current_dir(new_dir) {
            eprintln!("Failed to change directory: {}", e);
        }

        let config = "test_config.json";

        training(config)?;

        Ok(())
    }
}
