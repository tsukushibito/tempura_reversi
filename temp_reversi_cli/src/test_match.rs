use rand::prelude::*;
use rand::rng;
use rayon::prelude::*;
use temp_reversi_ai::ai_decider::AiDecider;
use temp_reversi_ai::evaluator::TempuraEvaluator;
// use temp_reversi_ai::strategy::NegaAlphaStrategy;
use temp_reversi_ai::strategy::NegaAlphaTTStrategy;
use temp_reversi_ai::strategy::Strategy;
use temp_reversi_core::Bitboard;
use temp_reversi_core::{Game, MoveDecider, Player};

pub fn run_test_match(num_games: usize, black_model_path: &str, white_model_path: &str) {
    // Create evaluators and strategies.
    let tempura_evaluator = TempuraEvaluator::new(black_model_path);
    let black_strategy = NegaAlphaTTStrategy::new(tempura_evaluator, 5, 0.0);
    // let black_strategy = NegaAlphaStrategy::new(tempura_evaluator, 5);
    let tempura_evaluator = TempuraEvaluator::new(white_model_path);
    let white_strategy = NegaAlphaTTStrategy::new(tempura_evaluator, 5, 0.0);
    // let white_strategy = NegaAlphaStrategy::new(tempura_evaluator, 5);

    // Run simulations in parallel.
    let (pattern_wins, phase_wins, draws) = (0..num_games)
        .into_par_iter()
        .map(|_| {
            let mut game = Game::<Bitboard>::default();
            // Create local AI deciders by cloning strategies.
            let mut local_pattern_ai = AiDecider::new(black_strategy.clone_box());
            let mut local_phase_ai = AiDecider::new(white_strategy.clone_box());
            let mut random_moves = 5;
            while !game.is_game_over() {
                if random_moves > 0 {
                    random_moves -= 1;
                    let valid_moves = game.valid_moves();
                    let random_move = valid_moves.choose(&mut rng());
                    if let Some(random_move) = random_move {
                        game.apply_move(*random_move).unwrap();
                    } else {
                        break;
                    }
                }
                let current_ai = if game.current_player() == Player::Black {
                    &mut local_pattern_ai
                } else {
                    &mut local_phase_ai
                };
                if let Some(chosen_move) = current_ai.select_move(&game) {
                    game.apply_move(chosen_move).unwrap();
                } else {
                    break;
                }
            }
            let (black_stones, white_stones) = game.current_score();
            if black_stones > white_stones {
                (1, 0, 0)
            } else if white_stones > black_stones {
                (0, 1, 0)
            } else {
                (0, 0, 1)
            }
        })
        .reduce(|| (0, 0, 0), |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2));

    println!("Test Match Results:");
    println!("Black wins: {}", pattern_wins);
    println!("White wins: {}", phase_wins);
    println!("Draws: {}", draws);
}
