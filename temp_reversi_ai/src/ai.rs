use rand::seq::SliceRandom;
use temp_reversi_core::{Game, Position};

/// Decide the next move for the given player using a random strategy.
pub fn decide_next_move(game: &Game) -> Option<Position> {
    let valid_moves = game.valid_moves();
    if valid_moves.is_empty() {
        None
    } else {
        // Choose a random move from the list of valid moves.
        let mut rng = rand::thread_rng();
        valid_moves.choose(&mut rng).cloned()
    }
}
