use crate::{
    ml::{Adam, Dataloader, LearnerBuilder, Model, Mse, StepLr},
    Config, ResultBoxErr, TempuraEvaluator,
};

pub fn training(config: &str) -> ResultBoxErr<()> {
    let config = Config::from_file(config)?;

    let evaluator = TempuraEvaluator::default();
    let input_size = evaluator.feature_size();
    let model = Model::new(input_size);

    let data_loader = Dataloader::new(
        config.training_game_records_path(),
        config.training.batch_size,
        true,
    )?;

    println!("Game records has loaded.");

    let optimizer = Adam::new(0.001, 0.9, 0.999, 1e-8);
    let loss_function = Mse::new();
    let lr_scheduler = StepLr::new(50, 0.1);

    let mut learner = LearnerBuilder::default()
        .model(model)
        .train_dataloader(data_loader)
        .optimizer(optimizer)
        .num_epochs(100)
        .loss_function(loss_function)
        .lr_scheduler(Some(lr_scheduler))
        .build()?;

    learner.fit()?;

    learner.model.save(config.training_output_path())?;

    Ok(())
}
