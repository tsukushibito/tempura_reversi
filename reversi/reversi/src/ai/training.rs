use crate::{BitBoard, Game, Position};

use super::{GameRecord, Pattern, PatternTable};

pub struct TrainingExample {
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
    let mut gradients = vec![0.0; pattern_table.scores.len()];

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
    pattern_table
        .scores
        .iter_mut()
        .zip(gradients.iter())
        .for_each(|(score, &g)| {
            *score -= alpha * g;
        });
}
