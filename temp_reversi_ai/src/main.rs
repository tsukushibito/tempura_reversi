use temp_reversi_ai::{
    evaluator::{PhaseAwareEvaluator, TempuraEvaluator},
    strategy::{NegaAlphaTTStrategy, NegaScoutStrategy, Strategy},
};
use temp_reversi_core::Game;

fn main() {
    let depth = 5;

    let mut game = Game::default();
    let evaluator = TempuraEvaluator::new("./gen0/models/temp_model.bin");
    let mut strategy =
        NegaScoutStrategy::new(evaluator.clone(), PhaseAwareEvaluator::default(), depth);

    let start = std::time::Instant::now();
    let mut visitied_nodes = 0;
    while !game.is_over() {
        let best_move = strategy.select_move(&game.board_state(), game.current_player());
        game.apply_move(best_move).unwrap();
        visitied_nodes += strategy.nega_scout.visited_nodes;
    }
    let elapsed = start.elapsed();
    println!(
        "[NegaScout] Elapsed: {:?}, visited nodes: {}",
        elapsed, visitied_nodes
    );

    let mut game = Game::default();
    let evaluator = TempuraEvaluator::new("./gen0/models/temp_model.bin");
    let mut strategy = NegaAlphaTTStrategy::new(evaluator.clone(), evaluator.clone(), depth);

    let start = std::time::Instant::now();
    let mut visitied_nodes = 0;
    while !game.is_over() {
        let best_move = strategy.select_move(&game.board_state(), game.current_player());
        game.apply_move(best_move).unwrap();
        visitied_nodes += strategy.nega_alpha_tt.visited_nodes;
    }
    let elapsed = start.elapsed();
    println!(
        "[NegaAlphaTT] Elapsed: {:?}, visited nodes: {}",
        elapsed, visitied_nodes
    );
}
