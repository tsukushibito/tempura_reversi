use temp_reversi_ai::{
    evaluator::TempuraEvaluator,
    strategy::{NegaAlphaTTStrategy, NegaScoutStrategy2, Strategy},
};
use temp_reversi_core::Game;

fn main() {
    let depth = 10;

    let mut game = Game::default();
    let valid_moves = game.valid_moves();
    game.apply_move(valid_moves[0]).unwrap();
    let valid_moves = game.valid_moves();
    game.apply_move(valid_moves[0]).unwrap();
    let evaluator = TempuraEvaluator::new("../gen0/models/temp_model.bin");
    // let evaluator = TempuraEvaluator::new("m");
    let mut strategy = NegaAlphaTTStrategy::new(evaluator, depth, 0.0);

    let start = std::time::Instant::now();
    strategy.evaluate_and_decide(&game.board_state(), game.current_player());
    let elapsed = start.elapsed();
    println!("[NegaAlphaTT] Elapsed: {:?}", elapsed);
    assert!(
        strategy.visited_nodes > 0,
        "Visited nodes should be greater than 0."
    );
    println!("[NegaAlphaTT] Visited nodes: {}", strategy.visited_nodes);

    let mut game = Game::default();
    let valid_moves = game.valid_moves();
    game.apply_move(valid_moves[0]).unwrap();
    let valid_moves = game.valid_moves();
    game.apply_move(valid_moves[0]).unwrap();
    let mut strategy = NegaScoutStrategy2::new("../gen0/models/temp_model.bin", depth as usize);
    // let mut strategy = NegaScoutStrategy2::new("m", depth as usize);

    let start = std::time::Instant::now();
    strategy.evaluate_and_decide(&game.board_state(), game.current_player());
    let elapsed = start.elapsed();
    println!("[NegaScout2] Elapsed: {:?}", elapsed);
    assert!(
        strategy.nega_scout.visited_nodes > 0,
        "Visited nodes should be greater than 0."
    );
    println!(
        "[NegaScout2] Visited nodes: {}",
        strategy.nega_scout.visited_nodes
    );
}
