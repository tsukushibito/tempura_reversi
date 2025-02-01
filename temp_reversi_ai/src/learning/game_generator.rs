use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

use super::{GameDataset, GameRecord};
use crate::{ai_decider::AiDecider, strategy::Strategy};
use rayon::prelude::*;
use temp_reversi_core::{Game, MoveDecider, Player};

/// Runs self-play games in parallel using AI players and generates game records.
///
/// # Arguments
/// - `num_games`: Number of self-play games to generate.
/// - `black_strategy`: The strategy for the black player.
/// - `white_strategy`: The strategy for the white player.
///
/// # Returns
/// - `GameDataset` containing generated game records.
pub fn generate_self_play_data(
    num_games: usize,
    black_strategy: Box<dyn Strategy>,
    white_strategy: Box<dyn Strategy>,
) -> GameDataset {
    let records: Vec<GameRecord> = (0..num_games)
        .into_par_iter()
        .map(|_| {
            let mut game = Game::default();
            let mut black_ai = AiDecider::new(black_strategy.clone_box());
            let mut white_ai = AiDecider::new(white_strategy.clone_box());

            let mut moves: Vec<u8> = Vec::new();

            while !game.is_game_over() {
                let current_ai = if game.current_player() == Player::Black {
                    &mut black_ai
                } else {
                    &mut white_ai
                };

                if let Some(best_move) = current_ai.select_move(&game) {
                    moves.push(best_move.to_u8());
                    game.apply_move(best_move).unwrap();
                } else {
                    break;
                }
            }

            let (black_score, white_score) = game.current_score();
            GameRecord {
                moves,
                final_score: (black_score as u8, white_score as u8),
            }
        })
        .collect();

    GameDataset { records }
}

/// Generates self-play data and saves it to the specified file path.
///
/// # Arguments
/// - `num_games`: Number of self-play games to generate.
/// - `black_strategy`: The strategy for the black player.
/// - `white_strategy`: The strategy for the white player.
/// - `dataset_path`: Path to save the generated dataset.
///
/// # Returns
/// - `Result<(), String>` indicating success or error.
pub fn generate_and_save_self_play_data(
    num_games: usize,
    black_strategy: Box<dyn Strategy>,
    white_strategy: Box<dyn Strategy>,
    dataset_path: &str,
) -> Result<(), String> {
    println!("🔄 Generating {} self-play games...", num_games);

    let game_data = generate_self_play_data(num_games, black_strategy, white_strategy);
    println!("✅ {} games generated.", game_data.len());

    // Ensure the parent directory exists
    if let Some(parent) = Path::new(dataset_path).parent() {
        create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    // Save the game dataset in binary format
    let serialized = bincode::serialize(&game_data).map_err(|e| e.to_string())?;
    let mut file = File::create(dataset_path).map_err(|e| e.to_string())?;
    file.write_all(&serialized).map_err(|e| e.to_string())?;

    println!("💾 Dataset saved to {}", dataset_path);
    Ok(())
}
