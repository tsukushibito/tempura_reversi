use super::Strategy;
use rand::{prelude::*, rng};
use temp_reversi_core::{Board, Game, Position};

/// A random strategy that selects a move randomly from the list of valid moves.
pub struct RandomStrategy;

impl<B: Board> Strategy<B> for RandomStrategy {
    fn evaluate_and_decide(&mut self, game: &Game<B>) -> Option<Position> {
        let mut rng = rng();
        let valid_moves = game.valid_moves();
        valid_moves.choose(&mut rng).copied()
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
    fn test_random_strategy() {
        let game = Game::<Bitboard>::default();
        let mut strategy = RandomStrategy;

        let move_option = strategy.evaluate_and_decide(&game);
        assert!(
            move_option.is_some(),
            "RandomStrategy should return a valid move."
        );
    }
}
