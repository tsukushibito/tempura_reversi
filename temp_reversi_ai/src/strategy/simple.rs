use super::Strategy;
use temp_reversi_core::{Bitboard, Player, Position};

/// A simple strategy that selects the first valid move.
#[derive(Clone, Debug)]
pub struct SimpleStrategy;

impl Strategy for SimpleStrategy {
    fn select_move(&mut self, board: &Bitboard, player: Player) -> Option<Position> {
        board.valid_moves(player).into_iter().next()
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use temp_reversi_core::Game;

    #[test]
    fn test_simple_strategy() {
        let game = Game::default();
        let mut strategy = SimpleStrategy;

        let move_option = strategy.select_move(&game.board_state(), game.current_player());
        assert!(
            move_option.is_some(),
            "SimpleStrategy should return a valid move."
        );
    }
}
