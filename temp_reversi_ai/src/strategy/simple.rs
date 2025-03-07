use super::Strategy;
use temp_reversi_core::{Board, Game, Position};

/// A simple strategy that selects the first valid move.
pub struct SimpleStrategy;

impl<B: Board> Strategy<B> for SimpleStrategy {
    fn evaluate_and_decide(&mut self, game: &Game<B>) -> Option<Position> {
        game.valid_moves().into_iter().next()
    }

    fn clone_box(&self) -> Box<dyn Strategy<B>> {
        Box::new(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use temp_reversi_core::{Bitboard, Game};

    #[test]
    fn test_simple_strategy() {
        let game = Game::<Bitboard>::default();
        let mut strategy = SimpleStrategy;

        let move_option = strategy.evaluate_and_decide(&game);
        assert!(
            move_option.is_some(),
            "SimpleStrategy should return a valid move."
        );
    }
}
