use super::Strategy;
use rand::{prelude::*, rng};
use temp_reversi_core::{Bitboard, Player, Position};

/// A random strategy that selects a move randomly from the list of valid moves.
pub struct RandomStrategy;

impl Strategy for RandomStrategy {
    fn evaluate_and_decide(&mut self, board: &Bitboard, player: Player) -> Option<Position> {
        let mut rng = rng();
        let valid_moves = board.valid_moves(player);
        valid_moves.choose(&mut rng).copied()
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use temp_reversi_core::Game;

    #[test]
    fn test_random_strategy() {
        let game = Game::default();
        let mut strategy = RandomStrategy;

        let move_option = strategy.evaluate_and_decide(&game.board_state(), game.current_player());
        assert!(
            move_option.is_some(),
            "RandomStrategy should return a valid move."
        );
    }
}
