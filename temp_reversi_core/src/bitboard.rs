use std::fmt;

use crate::{player::*, position::*};

#[derive(Debug, Clone)]
pub struct Bitboard {
    black: u64, // Bitboard for black stones
    white: u64, // Bitboard for white stones
}

impl Default for Bitboard {
    /// Creates a new game board in the default initial state.
    fn default() -> Self {
        Self {
            black: D5.to_bit() | E4.to_bit(), // Initial black stones
            white: D4.to_bit() | E5.to_bit(), // Initial white stones
        }
    }
}

impl Bitboard {
    /// Directions for bitwise operations in the game (shift amount and mask).
    const DIRECTIONS: [(i32, u64); 8] = [
        (1, 0xfefefefefefefefe),  // Right
        (-1, 0x7f7f7f7f7f7f7f7f), // Left
        (8, 0xffffffffffffff00),  // Down
        (-8, 0x00ffffffffffffff), // Up
        (9, 0xfefefefefefefe00),  // Down-right
        (7, 0x7f7f7f7f7f7f7f00),  // Down-left
        (-9, 0x007f7f7f7f7f7f7f), // Up-left
        (-7, 0x00fefefefefefefe), // Up-right
    ];

    /// Creates a new Bitboard with the specified black and white stone positions.
    pub fn new(black: u64, white: u64) -> Self {
        Self { black, white }
    }

    /// Returns the current state of the bitboard as a tuple of black and white positions.
    pub fn bits(&self) -> (u64, u64) {
        (self.black, self.white)
    }

    /// Returns a list of valid moves for the specified player.
    ///
    /// # Arguments
    /// * `player` - The current player (Black or White).
    pub fn valid_moves(&self, player: Player) -> Vec<Position> {
        let bitmask = self.valid_moves_bitmask(player);
        self.bitmask_to_positions(bitmask)
    }

    /// Counts the number of stones for both black and white players.
    ///
    /// # Returns
    /// A tuple of (black stone count, white stone count).
    pub fn count_stones(&self) -> (usize, usize) {
        (
            self.black.count_ones() as usize,
            self.white.count_ones() as usize,
        )
    }

    /// Checks if the game is over. The game ends if neither player has any valid moves.
    pub fn is_game_over(&self) -> bool {
        self.valid_moves(Player::Black).is_empty() && self.valid_moves(Player::White).is_empty()
    }

    /// Applies a move to the board for the specified player.
    ///
    /// # Arguments
    /// * `position` - The position to place the stone.
    /// * `player` - The current player making the move.
    ///
    /// # Returns
    /// `Ok(())` if the move is valid and applied successfully, otherwise an error message.
    pub fn apply_move(&mut self, position: Position, player: Player) -> Result<(), &'static str> {
        let move_bit = position.to_bit();

        // Check if the position is already occupied.
        if self.black & move_bit != 0 || self.white & move_bit != 0 {
            return Err("Invalid move: position is already occupied");
        }

        let (player_bits, opponent_bits) = match player {
            Player::Black => (&mut self.black, &mut self.white),
            Player::White => (&mut self.white, &mut self.black),
        };

        // Calculate the stones to flip for the move.
        let flips = Self::get_flips_bits(move_bit, *player_bits, *opponent_bits);

        // If no stones can be flipped, the move is invalid.
        if flips == 0 {
            return Err("Invalid move: no stones to flip");
        }

        // Update the board with the move.
        *player_bits |= move_bit | flips;
        *opponent_bits &= !flips;

        Ok(())
    }

    /// Safely shifts bits in a specified direction, applying a mask to prevent invalid shifts.
    ///
    /// # Arguments
    /// * `bits` - The bitboard to shift.
    /// * `shift_amount` - The direction and amount to shift.
    /// * `mask` - The mask to apply after shifting.
    fn safe_shift(bits: u64, shift_amount: i32, mask: u64) -> u64 {
        let shifted = if shift_amount > 0 {
            bits << shift_amount
        } else {
            bits >> -shift_amount
        };
        shifted & mask
    }

    /// Calculates valid moves for the specified player as a bitmask.
    ///
    /// # Arguments
    /// * `player` - The current player.
    ///
    /// # Returns
    /// A bitmask of valid moves.
    fn valid_moves_bitmask(&self, player: Player) -> u64 {
        let (player_bits, opponent_bits) = match player {
            Player::Black => (self.black, self.white),
            Player::White => (self.white, self.black),
        };
        let empty = !(player_bits | opponent_bits);
        let mut valid_moves = 0u64;

        for &(shift_amount, mask) in &Self::DIRECTIONS {
            let mut tmp = Self::safe_shift(player_bits, shift_amount, mask) & opponent_bits;

            for _ in 0..6 {
                tmp |= Self::safe_shift(tmp, shift_amount, mask) & opponent_bits;
            }

            valid_moves |= Self::safe_shift(tmp, shift_amount, mask) & empty;
        }

        valid_moves
    }

    /// Calculates the stones that would be flipped if a move is applied.
    ///
    /// # Arguments
    /// * `move_bit` - The bit position of the move.
    /// * `player_bits` - The bitboard of the current player.
    /// * `opponent_bits` - The bitboard of the opponent.
    ///
    /// # Returns
    /// A bitmask of stones to be flipped.
    fn get_flips_bits(move_bit: u64, player_bits: u64, opponent_bits: u64) -> u64 {
        let mut flips = 0u64;

        for &(shift_amount, mask) in &Self::DIRECTIONS {
            let mut tmp_flips = 0;
            let mut tmp = Self::safe_shift(move_bit, shift_amount, mask) & opponent_bits;

            while tmp != 0 {
                tmp_flips |= tmp;
                tmp = Self::safe_shift(tmp, shift_amount, mask) & opponent_bits;
            }

            if Self::safe_shift(tmp_flips, shift_amount, mask) & player_bits != 0 {
                flips |= tmp_flips;
            }
        }

        flips
    }

    /// Converts a bitmask to a list of `Position` objects.
    ///
    /// # Arguments
    /// * `bitmask` - The bitmask representing positions.
    ///
    /// # Returns
    /// A vector of positions.
    fn bitmask_to_positions(&self, bitmask: u64) -> Vec<Position> {
        let mut positions = Vec::new();
        let mut bits = bitmask;

        while bits != 0 {
            let lsb = bits & (!bits + 1); // Extract the least significant bit
            if let Some(position) = Position::from_bit(lsb) {
                positions.push(position);
            }
            bits &= bits - 1; // Clear the least significant bit
        }

        positions
    }
}

impl std::fmt::Display for Bitboard {
    /// Displays the current board state as a string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  A B C D E F G H")?; // Column headers
        for row in 0..8 {
            write!(f, "{} ", row + 1)?; // Row numbers
            for col in 0..8 {
                let pos = 1 << (row * 8 + col);
                if self.black & pos != 0 {
                    write!(f, "B ")?; // Black stone
                } else if self.white & pos != 0 {
                    write!(f, "W ")?; // White stone
                } else {
                    write!(f, ". ")?; // Empty cell
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rand::{seq::SliceRandom, thread_rng};

    use super::*;
    use crate::position::{D4, D5, E4, E5};

    #[test]
    fn test_default() {
        let board = Bitboard::default();
        println!("{board}");
        assert_eq!(board.black, D5.to_bit() | E4.to_bit());
        assert_eq!(board.white, D4.to_bit() | E5.to_bit());
    }

    #[test]
    fn test_new() {
        let board = Bitboard::new(D5.to_bit() | E4.to_bit(), D4.to_bit() | E5.to_bit());
        assert_eq!(board.black, D5.to_bit() | E4.to_bit());
        assert_eq!(board.white, D4.to_bit() | E5.to_bit());
    }

    #[test]
    fn test_get_bitboard_states() {
        let board = Bitboard::new(D5.to_bit() | E4.to_bit(), D4.to_bit() | E5.to_bit());
        let (black, white) = board.bits();
        assert_eq!(black, D5.to_bit() | E4.to_bit());
        assert_eq!(white, D4.to_bit() | E5.to_bit());
    }

    #[test]
    fn test_count_stones() {
        let board = Bitboard::new(D5.to_bit() | E4.to_bit(), D4.to_bit() | E5.to_bit());
        let (black_count, white_count) = board.count_stones();
        assert_eq!(black_count, 2);
        assert_eq!(white_count, 2);
    }

    #[test]
    fn test_get_valid_moves_bitmask() {
        let black = D5.to_bit() | E4.to_bit();
        let white = D4.to_bit() | E5.to_bit();
        let board = Bitboard::new(black, white);

        let bitmask = board.valid_moves_bitmask(Player::Black);

        let bitmask_board = Bitboard::new(bitmask, 0);
        println!("[bitmask]");
        println!("{bitmask_board}");

        let expected = D3.to_bit() | C4.to_bit() | F5.to_bit() | E6.to_bit();
        let expected_board = Bitboard::new(expected, 0);
        println!("[expected]");
        println!("{expected_board}");

        assert_eq!(bitmask, expected);
    }

    #[test]
    fn test_valid_moves_corners() {
        // 左上隅のテスト
        {
            let black = A1.to_bit();
            let white = B1.to_bit() | A2.to_bit() | B2.to_bit();
            let board = Bitboard::new(black, white);
            println!("[Left-Top Corner Test - Board]");
            println!("{board}");

            let bitmask = board.valid_moves_bitmask(Player::Black);
            let bitmask_board = Bitboard::new(bitmask, 0);
            println!("[Left-Top Corner Test - Bitmask Board]");
            println!("{bitmask_board}");

            let expected = C1.to_bit() | A3.to_bit() | C3.to_bit();
            let expected_board = Bitboard::new(expected, 0);
            println!("[Left-Top Corner Test - Expected Board]");
            println!("{expected_board}");

            assert_eq!(bitmask, expected);
        }

        // 右上隅のテスト
        {
            let black = H1.to_bit();
            let white = G1.to_bit() | H2.to_bit() | G2.to_bit();
            let board = Bitboard::new(black, white);
            println!("[Right-Top Corner Test - Board]");
            println!("{board}");

            let bitmask = board.valid_moves_bitmask(Player::Black);
            let bitmask_board = Bitboard::new(bitmask, 0);
            println!("[Right-Top Corner Test - Bitmask Board]");
            println!("{bitmask_board}");

            let expected = F1.to_bit() | H3.to_bit() | F3.to_bit();
            let expected_board = Bitboard::new(expected, 0);
            println!("[Right-Top Corner Test - Expected Board]");
            println!("{expected_board}");

            assert_eq!(bitmask, expected);
        }

        // 左下隅のテスト
        {
            let black = A8.to_bit();
            let white = B8.to_bit() | A7.to_bit() | B7.to_bit();
            let board = Bitboard::new(black, white);
            println!("[Left-Bottom Corner Test - Board]");
            println!("{board}");

            let bitmask = board.valid_moves_bitmask(Player::Black);
            let bitmask_board = Bitboard::new(bitmask, 0);
            println!("[Left-Bottom Corner Test - Bitmask Board]");
            println!("{bitmask_board}");

            let expected = C8.to_bit() | A6.to_bit() | C6.to_bit();
            let expected_board = Bitboard::new(expected, 0);
            println!("[Left-Bottom Corner Test - Expected Board]");
            println!("{expected_board}");

            assert_eq!(bitmask, expected);
        }

        // 右下隅のテスト
        {
            let black = H8.to_bit();
            let white = G8.to_bit() | H7.to_bit() | G7.to_bit();
            let board = Bitboard::new(black, white);
            println!("[Right-Bottom Corner Test - Board]");
            println!("{board}");

            let bitmask = board.valid_moves_bitmask(Player::Black);
            let bitmask_board = Bitboard::new(bitmask, 0);
            println!("[Right-Bottom Corner Test - Bitmask Board]");
            println!("{bitmask_board}");

            let expected = F8.to_bit() | H6.to_bit() | F6.to_bit();
            let expected_board = Bitboard::new(expected, 0);
            println!("[Right-Bottom Corner Test - Expected Board]");
            println!("{expected_board}");

            assert_eq!(bitmask, expected);
        }
    }

    #[test]
    fn test_valid_moves_edges() {
        {
            let black = A3.to_bit();
            let white = A2.to_bit() | A4.to_bit() | B2.to_bit() | B3.to_bit() | B4.to_bit();
            let board = Bitboard::new(black, white);

            println!("[Left Edge Test - Board]");
            println!("{board}");

            let bitmask = board.valid_moves_bitmask(Player::Black);
            let bitmask_board = Bitboard::new(bitmask, 0);
            println!("[Left Edge Test - Bitmask Board]");
            println!("{bitmask_board}");

            let expected = A1.to_bit() | C1.to_bit() | C3.to_bit() | A5.to_bit() | C5.to_bit();
            let expected_board = Bitboard::new(expected, 0);
            println!("[Left Edge Test - Expected Board]");
            println!("{expected_board}");

            assert_eq!(bitmask, expected);
        }

        // 上端のテスト
        {
            let black = C1.to_bit();
            let white = B1.to_bit() | D1.to_bit() | B2.to_bit() | C2.to_bit() | D2.to_bit();
            let board = Bitboard::new(black, white);

            println!("[Top Edge Test - Board]");
            println!("{board}");

            let bitmask = board.valid_moves_bitmask(Player::Black);
            let bitmask_board = Bitboard::new(bitmask, 0);
            println!("[Top Edge Test - Bitmask Board]");
            println!("{bitmask_board}");

            let expected = A1.to_bit() | A3.to_bit() | C3.to_bit() | E1.to_bit() | E3.to_bit();
            let expected_board = Bitboard::new(expected, 0);
            println!("[Top Edge Test - Expected Board]");
            println!("{expected_board}");

            assert_eq!(bitmask, expected);
        }

        // 右端のテスト
        {
            let black = H3.to_bit();
            let white = H2.to_bit() | H4.to_bit() | G2.to_bit() | G3.to_bit() | G4.to_bit();
            let board = Bitboard::new(black, white);

            println!("[Right Edge Test - Board]");
            println!("{board}");

            let bitmask = board.valid_moves_bitmask(Player::Black);
            let bitmask_board = Bitboard::new(bitmask, 0);
            println!("[Right Edge Test - Bitmask Board]");
            println!("{bitmask_board}");

            let expected = H1.to_bit() | F1.to_bit() | F3.to_bit() | H5.to_bit() | F5.to_bit();
            let expected_board = Bitboard::new(expected, 0);
            println!("[Right Edge Test - Expected Board]");
            println!("{expected_board}");

            assert_eq!(bitmask, expected);
        }

        // 下端のテスト
        {
            let black = C8.to_bit();
            let white = B8.to_bit() | D8.to_bit() | B7.to_bit() | C7.to_bit() | D7.to_bit();
            let board = Bitboard::new(black, white);

            println!("[Bottom Edge Test - Board]");
            println!("{board}");

            let bitmask = board.valid_moves_bitmask(Player::Black);
            let bitmask_board = Bitboard::new(bitmask, 0);
            println!("[Bottom Edge Test - Bitmask Board]");
            println!("{bitmask_board}");

            let expected = A8.to_bit() | A6.to_bit() | C6.to_bit() | E8.to_bit() | E6.to_bit();
            let expected_board = Bitboard::new(expected, 0);
            println!("[Bottom Edge Test - Expected Board]");
            println!("{expected_board}");

            assert_eq!(bitmask, expected);
        }
    }

    #[test]
    fn test_valid_moves_edges_2() {
        {
            let black = B3.to_bit();
            let white = A2.to_bit() | A3.to_bit() | A4.to_bit();
            let board = Bitboard::new(black, white);

            println!("[Left Edge Test - Board]");
            println!("{board}");

            let bitmask = board.valid_moves_bitmask(Player::Black);
            let bitmask_board = Bitboard::new(bitmask, 0);
            println!("[Left Edge Test - Bitmask Board]");
            println!("{bitmask_board}");

            let expected = 0;
            let expected_board = Bitboard::new(expected, 0);
            println!("[Left Edge Test - Expected Board]");
            println!("{expected_board}");

            assert_eq!(bitmask, expected);
        }

        {
            let black = C2.to_bit();
            let white = B1.to_bit() | C1.to_bit() | D1.to_bit();
            let board = Bitboard::new(black, white);

            println!("[Top Edge Test - Board]");
            println!("{board}");

            let bitmask = board.valid_moves_bitmask(Player::Black);
            let bitmask_board = Bitboard::new(bitmask, 0);
            println!("[Top Edge Test - Bitmask Board]");
            println!("{bitmask_board}");

            let expected = 0;
            let expected_board = Bitboard::new(expected, 0);
            println!("[Top Edge Test - Expected Board]");
            println!("{expected_board}");

            assert_eq!(bitmask, expected);
        }

        {
            let black = G3.to_bit();
            let white = H2.to_bit() | H3.to_bit() | H4.to_bit();
            let board = Bitboard::new(black, white);

            println!("[Right Edge Test - Board]");
            println!("{board}");

            let bitmask = board.valid_moves_bitmask(Player::Black);
            let bitmask_board = Bitboard::new(bitmask, 0);
            println!("[Right Edge Test - Bitmask Board]");
            println!("{bitmask_board}");

            let expected = 0;
            let expected_board = Bitboard::new(expected, 0);
            println!("[Right Edge Test - Expected Board]");
            println!("{expected_board}");

            assert_eq!(bitmask, expected);
        }

        {
            let black = C7.to_bit();
            let white = B8.to_bit() | C8.to_bit() | D8.to_bit();
            let board = Bitboard::new(black, white);

            println!("[Bottom Edge Test - Board]");
            println!("{board}");

            let bitmask = board.valid_moves_bitmask(Player::Black);
            let bitmask_board = Bitboard::new(bitmask, 0);
            println!("[Bottom Edge Test - Bitmask Board]");
            println!("{bitmask_board}");

            let expected = 0;
            let expected_board = Bitboard::new(expected, 0);
            println!("[Bottom Edge Test - Expected Board]");
            println!("{expected_board}");

            assert_eq!(bitmask, expected);
        }
    }

    #[test]
    fn test_apply_move() {
        let mut board = Bitboard::default();

        let position = E6;
        assert!(board.apply_move(position, Player::Black).is_ok());

        println!("[Board after E6 move by Black]");
        println!("{}", board);

        let (black_count, white_count) = board.count_stones();
        assert_eq!(black_count, 4);
        assert_eq!(white_count, 1);

        let position = F4;
        assert!(board.apply_move(position, Player::White).is_ok());

        println!("[Board after F5 move by White]");
        println!("{}", board);

        let (black_count, white_count) = board.count_stones();
        assert_eq!(black_count, 3);
        assert_eq!(white_count, 3);

        let black_stones = [C2, D2, B3, C3, D3, C4, D4, E4, D5, E5, F5];
        let black_bits = black_stones.iter().fold(0, |bits, s| bits | s.to_bit());
        let white_stones = [A1, B1, C1, D1, E1, A2, B2, A3];
        let white_bits = white_stones.iter().fold(0, |bits, s| bits | s.to_bit());
        let mut board = Bitboard::new(black_bits, white_bits);
        let valid_moves_bit = board.valid_moves_bitmask(Player::White);
        let test = Bitboard::new(valid_moves_bit, 0);
        println!("[Test]");
        println!("{}", board);
        println!("{}", test);
        board.valid_moves(Player::White).iter().for_each(|m| {
            let r = board.apply_move(*m, Player::White);
            println!("[{m}]{r:?}");
        });

        let black = C3.to_bit() | D4.to_bit();
        let white = A1.to_bit() | B2.to_bit() | E5.to_bit();
        let board = Bitboard::new(black, white);
        println!("{board}");
        let bitmask = board.valid_moves_bitmask(Player::Black);
        let expected = F6.to_bit();
        assert_eq!(bitmask, expected);
    }

    #[test]
    fn test_apply_move_invalid_position() {
        let mut board = Bitboard::default();

        let position = Position { row: 3, col: 3 }; // D4
        assert!(board.apply_move(position, Player::Black).is_err());
    }

    #[test]
    fn test_apply_move_no_flips() {
        let mut board = Bitboard::default();

        let position = Position { row: 0, col: 0 }; // A1
        assert!(board.apply_move(position, Player::Black).is_err());
    }

    #[test]
    fn test_get_flips_bits() {
        let black = D5.to_bit() | E4.to_bit();
        let white = D4.to_bit() | E5.to_bit();

        let move_bit = E6.to_bit();
        let flips = Bitboard::get_flips_bits(move_bit, black, white);

        let expected_flips = E5.to_bit();
        println!("Flips: {:#018x}, Expected: {:#018x}", flips, expected_flips);
        assert_eq!(flips, expected_flips);

        let move_bit = C4.to_bit();
        let flips = Bitboard::get_flips_bits(move_bit, white, black);

        let expected_flips = 0x0000000000000000;
        println!("Flips: {:#018x}, Expected: {:#018x}", flips, expected_flips);
        assert_eq!(flips, expected_flips);
    }

    #[test]
    fn test_get_flips_bits_custom_board() {
        // Black stones at: A1, B1, C1, A2, A3
        // White stones at: B2, C2, B3
        let black =
            A1.to_bit() | B1.to_bit() | C1.to_bit() | D1.to_bit() | A2.to_bit() | A3.to_bit();
        let white = B2.to_bit() | C2.to_bit() | B3.to_bit();
        let board = Bitboard::new(black, white);
        println!("{board}");

        let move_bit = D2.to_bit();
        let flips = Bitboard::get_flips_bits(move_bit, black, white);

        let expected_flips = B2.to_bit() | C2.to_bit();

        println!(
            "Move: D2, Flips: {:#018x}, Expected: {:#018x}",
            flips, expected_flips
        );
        assert_eq!(
            flips, expected_flips,
            "Flips mismatch: expected {:#018x}, got {:#018x}",
            expected_flips, flips
        );

        let move_bit = D3.to_bit();
        let flips = Bitboard::get_flips_bits(move_bit, black, white);
        let expected_flips = C2.to_bit();

        println!(
            "Move: D3, Flips: {:#018x}, Expected: {:#018x}",
            flips, expected_flips
        );
        assert_eq!(
            flips, expected_flips,
            "Flips mismatch: expected {:#018x}, got {:#018x}",
            expected_flips, flips
        );

        let move_bit = C3.to_bit();
        let flips = Bitboard::get_flips_bits(move_bit, black, white);
        let expected_flips = B2.to_bit() | C2.to_bit() | B3.to_bit();

        let debug = Bitboard::new(flips, 0);
        println!("{debug}");
        println!(
            "Move: C3, Flips: {:#018x}, Expected: {:#018x}",
            flips, expected_flips
        );
        assert_eq!(
            flips, expected_flips,
            "Flips mismatch: expected {:#018x}, got {:#018x}",
            expected_flips, flips
        );

        let move_bit = A4.to_bit();
        let flips = Bitboard::get_flips_bits(move_bit, black, white);
        let expected_flips = C2.to_bit() | B3.to_bit();

        println!(
            "Move: A4, Flips: {:#018x}, Expected: {:#018x}",
            flips, expected_flips
        );
        assert_eq!(
            flips, expected_flips,
            "Flips mismatch: expected {:#018x}, got {:#018x}",
            expected_flips, flips
        );
    }

    #[test]
    fn test_random_simulation() {
        let mut board = Bitboard::default();
        let mut rng = thread_rng();

        let mut current_player = Player::Black;

        for _ in 0..60 {
            let valid_moves = board.valid_moves(current_player);

            if valid_moves.is_empty() {
                current_player = match current_player {
                    Player::Black => Player::White,
                    Player::White => Player::Black,
                };
                if board.valid_moves(current_player).is_empty() {
                    println!("No more valid moves. Game over.");
                    break;
                }
                continue;
            }

            let chosen_move = valid_moves
                .choose(&mut rng)
                .expect("Valid move selection failed");

            assert!(
                board.apply_move(*chosen_move, current_player).is_ok(),
                "Failed to apply move"
            );

            println!("[After {:?} places at {:?}]", current_player, chosen_move);
            println!("{}", board);

            current_player = match current_player {
                Player::Black => Player::White,
                Player::White => Player::Black,
            };
        }

        let (black_count, white_count) = board.count_stones();
        println!(
            "Final counts: Black = {}, White = {}",
            black_count, white_count
        );

        assert!(black_count + white_count <= 64, "Total stones exceed 64!");
    }
}
