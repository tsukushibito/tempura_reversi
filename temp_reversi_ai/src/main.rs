use temp_reversi_ai::{
    evaluator::TempuraEvaluator,
    strategy::{NegaAlphaTTStrategy, NegaScoutStrategy, Strategy},
};
use temp_reversi_core::Game;

fn main() {
    let depth = 5;

    let mut game = Game::default();
    let evaluator = TempuraEvaluator::new("./gen0/models/temp_model.bin");
    let mut strategy = NegaAlphaTTStrategy::new(evaluator, depth);

    let start = std::time::Instant::now();
    while !game.is_game_over() {
        let best_move = strategy.evaluate_and_decide(&game.board_state(), game.current_player());
        if let Some(best_move) = best_move {
            game.apply_move(best_move).unwrap();
        } else {
            break;
        }
    }
    let elapsed = start.elapsed();
    println!("Elapsed: {:?}", elapsed);
}
