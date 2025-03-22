use crate::cli_display::cli_display;
use crate::cli_player::CliPlayer;
use std::io::{self, Write};
use temp_reversi_ai::ai_player::AiPlayer;
use temp_reversi_ai::evaluator::{PhaseAwareEvaluator, TempuraEvaluator};
use temp_reversi_ai::strategy::NegaScoutStrategy;
use temp_reversi_core::{Game, GamePlayer, Player};

pub fn play_game() {
    let mut input = String::new();
    // Prompt for Black player's type.
    println!("Select player for Black (human/ai):");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let black_choice = input.trim().to_lowercase();

    // Prompt for White player's type.
    println!("Select player for White (human/ai):");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let white_choice = input.trim().to_lowercase();

    // Initialize deciders.
    let mut black_decider: Box<dyn GamePlayer> = if black_choice == "human" {
        Box::new(CliPlayer)
    } else {
        let evaluator = TempuraEvaluator::new("gen0/models/best_model.bin");
        let strategy = NegaScoutStrategy::new(evaluator, PhaseAwareEvaluator::default(), 8);
        Box::new(AiPlayer::new(Box::new(strategy)))
    };

    let mut white_decider: Box<dyn GamePlayer> = if white_choice == "human" {
        Box::new(CliPlayer)
    } else {
        let evaluator = TempuraEvaluator::new("gen0/models/best_model.bin");
        let strategy = NegaScoutStrategy::new(evaluator, PhaseAwareEvaluator::default(), 8);
        Box::new(AiPlayer::new(Box::new(strategy)))
    };

    // Create game and loop until game over.
    let mut game = Game::default();
    while !game.is_game_over() {
        cli_display(&game);
        let current_decider: &mut dyn GamePlayer = if game.current_player() == Player::Black {
            &mut *black_decider
        } else {
            &mut *white_decider
        };
        let selected_move = current_decider.select_move(&game);
        game.apply_move(selected_move).expect("Invalid move");
    }
    // Final display.
    cli_display(&game);
    println!("Game over!");
}
