use std::sync::Arc;

use super::{GameDataset, GameRecord};
use crate::{ai_decider::AiDecider, strategy::Strategy, utils::ProgressReporter};
use rayon::prelude::*;
use temp_reversi_core::{Game, MoveDecider, Player};

/// Runs self-play games in parallel using AI players and generates game records.
///
/// # Arguments
/// - `num_games`: Number of self-play games to generate.
/// - `black_strategy`: The strategy for the black player.
/// - `white_strategy`: The strategy for the white player.
/// - `reporter`: Optional progress reporter for tracking progress.
///
/// # Returns
/// - `GameDataset` containing generated game records.
pub fn generate_game_dataset(
    num_games: usize,
    black_strategy: Box<dyn Strategy>,
    white_strategy: Box<dyn Strategy>,
    reporter: Option<Arc<dyn ProgressReporter + Send + Sync>>, // ProgressReporter を共有
) -> GameDataset {
    if let Some(r) = &reporter {
        r.on_start(num_games);
    }

    let records: Vec<GameRecord> = (0..num_games)
        .into_par_iter()
        .map(|_| {
            let mut game = Game::default();
            let mut black_ai = AiDecider::new(black_strategy.clone_box());
            let mut white_ai = AiDecider::new(white_strategy.clone_box());

            while !game.is_game_over() {
                let current_ai = if game.current_player() == Player::Black {
                    &mut black_ai
                } else {
                    &mut white_ai
                };

                if let Some(best_move) = current_ai.select_move(&game) {
                    game.apply_move(best_move).unwrap();
                } else {
                    break;
                }
            }

            if let Some(r) = &reporter {
                r.on_progress(1, num_games, None);
            }

            GameRecord::new(&game)
        })
        .collect();

    if let Some(r) = &reporter {
        r.on_complete();
    }

    GameDataset { records }
}
