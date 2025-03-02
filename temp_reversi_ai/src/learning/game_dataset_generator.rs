use std::sync::Arc;

use super::{GameDataset, GameRecord};
use crate::{ai_decider::AiDecider, strategy::Strategy, utils::ProgressReporter};
use rayon::prelude::*;
use temp_reversi_core::{Game, MoveDecider};

/// Runs self-play games in parallel using AI players and generates game records.
///
/// # Arguments
/// - `num_games`: Number of self-play games to generate.
/// - `strategy`: The strategy for the player.
/// - `reporter`: Optional progress reporter for tracking progress.
///
/// # Returns
/// - `GameDataset` containing generated game records.
pub fn generate_game_dataset(
    num_games: usize,
    strategy: Box<dyn Strategy>,
    reporter: Option<Arc<dyn ProgressReporter + Send + Sync>>, // ProgressReporter を共有
) -> GameDataset {
    if let Some(r) = &reporter {
        r.on_start(num_games);
    }

    let records: Vec<GameRecord> = (0..num_games)
        .into_par_iter()
        .map(|_| {
            let mut game = Game::default();
            let mut ai = AiDecider::new(strategy.clone_box());

            while !game.is_game_over() {
                if let Some(best_move) = ai.select_move(&game) {
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
