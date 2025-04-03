use crate::{Game, Player, Position};

pub trait GamePlayer {
    fn select_move(&mut self, game: &Game) -> Position;
}

/// Main game loop for Reversi, allowing for human or AI players.
pub fn run_game<D1, D2>(
    mut black_player: D1,
    mut white_decider: D2,
    mut display: impl FnMut(&Game),
) -> Result<(), String>
where
    D1: GamePlayer,
    D2: GamePlayer,
{
    let mut game = Game::default();

    loop {
        display(&game);

        // Determine the move (either by human input or AI)
        let current_player = game.current_player();
        let position = match current_player {
            Player::Black => black_player.select_move(&game),
            Player::White => white_decider.select_move(&game),
        };

        if game.is_valid_move(position) {
            game.apply_move(position)?;
        } else {
            return Err(format!("Invalid move: {:?}", position));
        }

        // Check if the game is over
        if game.is_over() {
            display(&game);
            break;
        }
    }

    Ok(())
}
