use crate::bitboard::Bitboard;
use crate::player::Player;
use crate::position::Position;

/// Struct to manage the overall state of an Othello game.
#[derive(Debug)]
pub struct Game {
    /// Current game board.
    board: Bitboard,
    /// Current player (Black or White).
    current_player: Player,
    /// History of moves.
    moves: Vec<Position>,
}

impl Default for Game {
    /// Creates a new game in its default initial state.
    fn default() -> Self {
        Self {
            board: Default::default(),
            current_player: Player::Black,
            moves: Vec::new(),
        }
    }
}

impl Game {
    /// Creates a new game with the specified board state and current player.
    ///
    /// # Arguments
    /// * `board` - Initial board state.
    /// * `current_player` - Initial player to start the game.
    pub fn new(board: Bitboard, current_player: Player) -> Self {
        Self {
            board,
            current_player,
            moves: Vec::new(),
        }
    }

    /// Returns the current player.
    pub fn current_player(&self) -> Player {
        self.current_player
    }

    /// Gets the valid moves for the current player.
    ///
    /// # Returns
    /// A list of valid moves (`Vec<Position>`).
    pub fn valid_moves(&self) -> Vec<Position> {
        self.board.valid_moves(self.current_player)
    }

    /// Checks if a move at the specified position is valid.
    ///
    /// # Arguments
    /// * `position` - Position to check.
    ///
    /// # Returns
    /// `true` if the move is valid, otherwise `false`.
    pub fn is_valid_move(&self, position: Position) -> bool {
        self.valid_moves().contains(&position)
    }

    /// Applies the specified move and switches the turn.
    ///
    /// # Arguments
    /// * `position` - The position where the move is applied.
    ///
    /// # Returns
    /// - `Ok(())` if the move was successfully applied.
    /// - `Err(&str)` if the move is invalid.
    pub fn apply_move(&mut self, position: Position) -> Result<(), &'static str> {
        if !self.is_valid_move(position) {
            return Err("Invalid move");
        }

        self.board.apply_move(position, self.current_player)?;
        self.moves.push(position);
        self.switch_turn();

        if self.valid_moves().is_empty() {
            self.switch_turn();
        }

        Ok(())
    }

    /// Checks if the game is over.
    ///
    /// # Returns
    /// `true` if the game is over, otherwise `false`.
    pub fn is_game_over(&self) -> bool {
        self.board.is_game_over()
    }

    /// Determines the winner of the game.
    ///
    /// # Returns
    /// - `Ok(Some(Player))` if there is a winner.
    /// - `Ok(None)` if the game is a draw.
    /// - `Err(&str)` if the game is not yet over.
    pub fn winner(&self) -> Result<Option<Player>, &'static str> {
        if !self.is_game_over() {
            return Err("Game is not over yet");
        }

        let (black_count, white_count) = self.board.count_stones();
        if black_count > white_count {
            Ok(Some(Player::Black))
        } else if white_count > black_count {
            Ok(Some(Player::White))
        } else {
            Ok(None) // Draw
        }
    }

    /// Gets the current score of the game.
    ///
    /// # Returns
    /// A tuple `(number_of_black_stones, number_of_white_stones)`.
    pub fn current_score(&self) -> (usize, usize) {
        self.board.count_stones()
    }

    /// Returns the current state of the board.
    pub fn board_state(&self) -> &Bitboard {
        &self.board
    }

    /// Returns the history of moves.
    pub fn history(&self) -> &Vec<Position> {
        &self.moves
    }

    /// Switches the turn to the other player. (Internal use only)
    fn switch_turn(&mut self) {
        self.current_player = self.current_player.opponent();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::*;

    #[test]
    fn test_game_initialization() {
        // Test if the default game state is correctly initialized.
        let game = Game::default();

        // Verify the initial stone counts.
        let (black_count, white_count) = game.current_score();
        assert_eq!(black_count, 2);
        assert_eq!(white_count, 2);

        // Verify the current player.
        assert_eq!(game.current_player(), Player::Black);
    }

    #[test]
    fn test_valid_moves() {
        // Test valid moves for the initial state.
        let game = Game::default();
        let valid_moves = game.valid_moves();

        // Verify the expected valid moves.
        assert!(valid_moves.contains(&Position::D3));
        assert!(valid_moves.contains(&Position::C4));
        assert!(valid_moves.contains(&Position::F5));
        assert!(valid_moves.contains(&Position::E6));
        assert_eq!(valid_moves.len(), 4);
    }

    #[test]
    fn test_apply_move_and_turn_switch() {
        // Test if a move is applied correctly and turn switches.
        let mut game = Game::default();

        // Black places at D3.
        assert!(game.apply_move(Position::D3).is_ok());
        assert_eq!(game.current_player(), Player::White);

        // Verify the board state.
        let (black_count, white_count) = game.current_score();
        assert_eq!(black_count, 4);
        assert_eq!(white_count, 1);
    }

    #[test]
    fn test_game_over_and_winner() {
        // Test game-over logic and determining the winner.
        let game = Game::new(
            Bitboard::new(0xffffffffff000000, 0x0000000000ffffff),
            Player::Black,
        );

        // Verify if the game is over.
        assert!(game.is_game_over());

        // Check the winner.
        match game.winner() {
            Ok(Some(Player::Black)) => (),
            _ => panic!("Expected Black to win"),
        }
    }
}
