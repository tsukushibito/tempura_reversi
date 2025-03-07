use crate::{Board, Game, Player, Position};

pub trait MoveDecider<B: Board> {
    fn select_move(&mut self, game: &Game<B>) -> Option<Position>;
}

/// Main game loop for Reversi, allowing for human or AI players.
pub fn run_game<D1, D2, B>(
    mut black_decider: D1,
    mut white_decider: D2,
    mut display: impl FnMut(&Game<B>),
) -> Result<(), String>
where
    D1: MoveDecider<B>,
    D2: MoveDecider<B>,
    B: Board,
{
    let mut game = Game::default();

    loop {
        display(&game);

        // Determine the move (either by human input or AI)
        let current_player = game.current_player();
        let position = match current_player {
            Player::Black => black_decider.select_move(&game),
            Player::White => white_decider.select_move(&game),
        };

        if let Some(position) = position {
            if game.is_valid_move(position) {
                game.apply_move(position)?;
            } else {
                return Err(format!("Invalid move: {:?}", position));
            }
        } else {
            println!(
                "No valid moves for {:?}. Skipping turn.",
                game.current_player()
            );
        }

        // Check if the game is over
        if game.is_game_over() {
            display(&game);
            break;
        }
    }

    Ok(())
}
