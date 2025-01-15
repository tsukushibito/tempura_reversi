use crate::{
    ml::{Adam, Dataloader, LearnerBuilder, Model, Mse, StepLr},
    Config, ResultBoxErr, TempuraEvaluator,
};

pub fn training(config: &str) -> ResultBoxErr<()> {
    println!("config: {}", config);
    let config = Config::from_file(config)?;

    let evaluator = TempuraEvaluator::default();
    let input_size = evaluator.feature_size();
    let model = Model::new(input_size);

    println!("base_path: {}", config.base_path);

    let data_loader = Dataloader::new(
        config.training_data_for_training_path(),
        config.training.batch_size,
        true,
    )?;

    println!("Game records has loaded.");

    let optimizer = Adam::new(0.001, 0.9, 0.999, 1e-8);
    let loss_function = Mse::new();
    let lr_scheduler = StepLr::new(50, 0.1);

    let mut learner = LearnerBuilder::<Adam, StepLr, Mse>::default()
        .model(model)
        .train_dataloader(data_loader)
        .optimizer(optimizer)
        .num_epochs(config.training.epochs)
        .loss_function(loss_function)
        .lr_scheduler(None)
        .build()?;

    learner.fit()?;

    learner.model.save(config.training_output_path())?;

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
