use temp_reversi_ai::{
    evaluator::TempuraEvaluator,
    strategy::{NegaScoutStrategy, Strategy},
};
use temp_reversi_core::Game;

fn main() {
    let depth = 5;

    let mut game = Game::default();
    let evaluator = TempuraEvaluator::new("./gen0/models/temp_model.bin");
    // let mut strategy = NegaAlphaTTStrategy::new(evaluator, depth);
    let mut strategy = NegaScoutStrategy::new(evaluator, depth);

    let start = std::time::Instant::now();
    let mut visitied_nodes = 0;
    while !game.is_game_over() {
        let best_move = strategy.select_move(&game.board_state(), game.current_player());
        if let Some(best_move) = best_move {
            game.apply_move(best_move).unwrap();
        } else {
            break;
        }
        // visitied_nodes += strategy.nega_alpha_tt.visited_nodes;
        visitied_nodes += strategy.nega_scout.visited_nodes;
    }
    let elapsed = start.elapsed();
    println!("Elapsed: {:?}, visited nodes: {}", elapsed, visitied_nodes);
}
