use rand::seq::SliceRandom;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{BitBoard, Game, Position};

use super::{GameRecord, Pattern, PatternTable};

pub struct HyperParameter {
    pub alpha: f32,
    pub max_iters: usize,
    pub tolerance: f32,
    pub batch_size: usize,
    pub beta1: f32,
    pub beta2: f32,
    pub epsilon: f32,
}

pub fn train_pattern_table(
    pattern_table: &mut PatternTable,
    records: &[GameRecord],
    hyper_param: &HyperParameter,
) {
    let examples = extract_training_data(records, pattern_table.patterns());

    // mini_batch_gradient_descent(
    //     &examples,
    //     pattern_table,
    //     hyper_param.alpha,
    //     hyper_param.max_iters,
    //     hyper_param.tolerance,
    //     hyper_param.batch_size,
    // );

    mini_batch_gradient_descent_adam(&examples, pattern_table, hyper_param);
}

#[derive(Clone)]
struct TrainingExample {
    pub board: BitBoard,
    pub features: Vec<f32>,
    pub label: f32,
}

fn extract_training_data(records: &[GameRecord], patterns: &[Pattern]) -> Vec<TrainingExample> {
    let mut training_data = Vec::new();

    // for record in records {
    //     let diff = record.black_score as i32 - record.white_score as i32;
    //     let label = diff as f32;

    //     let mut game = Game::initial();

    //     for i in 0..=record.moves.len() {
    //         let board = BitBoard::from_board(game.board());

    //         let features: Vec<f32> = patterns
    //             .iter()
    //             .flat_map(|pattern| pattern.feature(&board).into_iter())
    //             .collect();

    //         training_data.push(TrainingExample {
    //             board,
    //             label,
    //             features,
    //         });

    //         if i >= record.moves.len() {
    //             break;
    //         }

    //         let pos = Position::from_index(record.moves[i].into());
    //         let _ = game.progress(game.current_player(), pos);
    //     }
    // }

    training_data
}

fn compute_mse(examples: &[TrainingExample], pattern_table: &PatternTable) -> f32 {
    let total_error = examples
        .par_iter()
        .map(|example| {
            let predicted = pattern_table.evaluate(&example.board);
            let error = predicted - example.label;
            error * error
        })
        .sum::<f32>();

    total_error / examples.len() as f32
}

fn compute_gradients(examples: &[TrainingExample], pattern_table: &PatternTable) -> Vec<f32> {
    let len = pattern_table.scores().len();
    let m = examples.len() as f32;

    // par_iterで並列イテレーション
    let gradients = examples
        .par_iter()
        .map(|example| {
            let pred = pattern_table.evaluate(&example.board);
            let error = pred - example.label;
            let features = pattern_table.features(&example.board);

            // このスレッド内で計算したpartial gradientを格納
            let mut local_grad = vec![0.0; len];
            for (index, &feature) in features.iter().enumerate() {
                local_grad[index] = error * feature;
            }
            local_grad
        })
        // スレッドごとの結果をreduceで集計
        .reduce(
            || vec![0.0; len],
            |mut acc, vec| {
                for (a, v) in acc.iter_mut().zip(vec.iter()) {
                    *a += v;
                }
                acc
            },
        );

    // 2.0/mでスケール
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

fn batch_gradient_descent(
    examples: &[TrainingExample],
    pattern_table: &mut PatternTable,
    alpha: f32,
    max_iters: usize,
    tolerance: f32,
) {
    let mut prev_mse = compute_mse(examples, pattern_table);

    for epoch in 0..max_iters {
        let gradients = compute_gradients(examples, pattern_table);
        update_scores(pattern_table, &gradients, alpha);

        let mse = compute_mse(examples, pattern_table);
        println!("Epoch {}: MSE = {}", epoch + 1, mse);

        if (prev_mse - mse).abs() < tolerance {
            println!("収束条件を満たしたため、トレーニングを終了します。");
            break;
        }
        prev_mse = mse;
    }
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
        shuffled.shuffle(&mut rng);

        for batch in shuffled.chunks(batch_size) {
            let gradients = compute_gradients(batch, pattern_table);
            update_scores(pattern_table, &gradients, alpha);
        }

        let mse = compute_mse(examples, pattern_table);
        println!("Epoch {}: MSE = {}", epoch + 1, mse);

        if (prev_mse - mse).abs() < tolerance {
            println!("収束条件を満たしたため、トレーニングを終了します。");
            break;
        }
        prev_mse = mse;
    }
}

fn mini_batch_gradient_descent_adam(
    examples: &[TrainingExample],
    pattern_table: &mut PatternTable,
    hyper_param: &HyperParameter,
) {
    let mut prev_mse = compute_mse(examples, pattern_table);
    let mut rng = rand::thread_rng();
    let mut shuffled = examples.to_vec();

    // Adam用モーメント初期化
    let len = pattern_table.scores().len();
    let mut m = vec![0.0; len];
    let mut v = vec![0.0; len];
    let mut t = 0; // 時間ステップ

    for epoch in 0..hyper_param.max_iters {
        shuffled.shuffle(&mut rng);

        for batch in shuffled.chunks(hyper_param.batch_size) {
            let gradients = compute_gradients(batch, pattern_table);
            t += 1;
            adam_update_scores(
                pattern_table,
                &gradients,
                &mut m,
                &mut v,
                t,
                hyper_param.alpha,
                hyper_param.beta1,
                hyper_param.beta2,
                hyper_param.epsilon,
            );
        }

        let mse = compute_mse(examples, pattern_table);
        println!("Epoch {}: MSE = {}", epoch + 1, mse);

        if (prev_mse - mse).abs() < hyper_param.tolerance {
            println!("収束条件を満たしたため、トレーニングを終了します。");
            break;
        }
        prev_mse = mse;
    }
}

fn adam_update_scores(
    pattern_table: &mut PatternTable,
    gradients: &[f32],
    m: &mut [f32],
    v: &mut [f32],
    t: usize,
    alpha: f32,
    beta1: f32,
    beta2: f32,
    epsilon: f32,
) {
    let mut scores = pattern_table.scores().clone();

    for i in 0..scores.len() {
        let g = gradients[i];

        // mとvを更新
        m[i] = beta1 * m[i] + (1.0 - beta1) * g;
        v[i] = beta2 * v[i] + (1.0 - beta2) * (g * g);

        // バイアス補正
        let m_hat = m[i] / (1.0 - beta1.powi(t as i32));
        let v_hat = v[i] / (1.0 - beta2.powi(t as i32));

        // パラメータ更新
        scores[i] -= alpha * m_hat / (v_hat.sqrt() + epsilon);
    }

    pattern_table.set_scores(&scores);
}
