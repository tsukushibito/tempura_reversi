use rand::seq::SliceRandom;

use crate::{BitBoard, Game, Position};

use super::{GameRecord, PatternTable};

pub struct HyperParameter {
    pub alpha: f32,
    pub max_iters: usize,
    pub tolerance: f32,
    pub batch_size: usize,
}

pub fn train_pattern_table(
    pattern_table: &mut PatternTable,
    records: &[GameRecord],
    hyper_param: &HyperParameter,
) {
    let examples = extract_training_data(records);

    mini_batch_gradient_descent(
        &examples,
        pattern_table,
        hyper_param.alpha,
        hyper_param.max_iters,
        hyper_param.tolerance,
        hyper_param.batch_size,
    );
}

#[derive(Clone)]
struct TrainingExample {
    pub board: BitBoard,
    pub label: f32,
}

fn extract_training_data(records: &[GameRecord]) -> Vec<TrainingExample> {
    let mut training_data = Vec::new();

    for record in records {
        let diff = record.black_score as i32 - record.white_score as i32;
        let label = diff as f32;

        let mut game = Game::initial();

        let board = BitBoard::from_board(game.board());

        training_data.push(TrainingExample { board, label });

        for &move_pos in &record.moves {
            let pos = Position::from_index(move_pos.into());
            let _ = game.progress(game.current_player(), pos);

            let board = BitBoard::from_board(game.board());

            training_data.push(TrainingExample { board, label });
        }
    }

    training_data
}

fn compute_mse(examples: &[TrainingExample], pattern_table: &PatternTable) -> f32 {
    let total_error = examples
        .iter()
        .map(|example| {
            let predicted = pattern_table.evaluate(&example.board);
            let error = predicted - example.label;
            error * error
        })
        .sum::<f32>();

    total_error / examples.len() as f32
}

fn compute_gradients(examples: &[TrainingExample], pattern_table: &PatternTable) -> Vec<f32> {
    let mut gradients = vec![0.0; pattern_table.scores().len()];

    examples.iter().for_each(|example| {
        let pred = pattern_table.evaluate(&example.board);
        let error = pred - example.label;
        let features = pattern_table.features(&example.board);

        features.iter().enumerate().for_each(|(index, feature)| {
            gradients[index] += error * feature;
        });
    });

    let m = examples.len() as f32;
    gradients.iter().map(|&g| (2.0 / m) * g).collect()
}

fn update_scores(pattern_table: &mut PatternTable, gradients: &[f32], alpha: f32) {
    let mut scores = pattern_table.scores().clone();
    scores
        .iter_mut()
        .zip(gradients.iter())
        .for_each(|(score, &g)| {
            *score -= alpha * g;
        });
    pattern_table.set_scores(&scores);
}

fn mini_batch_gradient_descent(
    examples: &[TrainingExample],
    pattern_table: &mut PatternTable,
    alpha: f32,
    max_iters: usize,
    tolerance: f32,
    batch_size: usize,
) {
    let mut prev_mse = compute_mse(examples, pattern_table);
    let mut rng = rand::thread_rng();
    let mut shuffled = examples.to_vec();

    for epoch in 0..max_iters {
        // データをシャッフル
        shuffled.shuffle(&mut rng);

        // ミニバッチに分割して処理
        for batch in shuffled.chunks(batch_size) {
            let gradients = compute_gradients(batch, pattern_table);
            update_scores(pattern_table, &gradients, alpha);
        }

        // エポック終了後にMSEを計算
        let mse = compute_mse(examples, pattern_table);
        println!("Epoch {}: MSE = {}", epoch + 1, mse);

        if (prev_mse - mse).abs() < tolerance {
            println!("収束条件を満たしたため、トレーニングを終了します。");
            break;
        }
        prev_mse = mse;
    }
}
