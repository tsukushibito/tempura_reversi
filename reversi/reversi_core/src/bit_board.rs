use crate::board::{Board, Color, Move, Position};

#[derive(Debug, Clone, Copy)]
enum Direction {
    East,      // 東
    West,      // 西
    South,     // 南
    North,     // 北
    SouthEast, // 南東
    NorthWest, // 北西
    SouthWest, // 南西
    NorthEast, // 北東
}

impl Direction {
    const DIRECTIONS: [Direction; 8] = [
        Direction::East,
        Direction::West,
        Direction::South,
        Direction::North,
        Direction::SouthEast,
        Direction::NorthWest,
        Direction::SouthWest,
        Direction::NorthEast,
    ];
}

const BOARD_SIZE: usize = 8;

#[derive(Debug, Clone)]
pub struct BitBoard {
    black: u64,
    white: u64,
}

fn get_shift_and_mask(dir: Direction) -> (i32, u64) {
    // Masks to prevent wrapping around the board edges
    let not_a_file = 0xfefefefefefefefeu64; // All bits except the A file (left edge)
    let not_h_file = 0x7f7f7f7f7f7f7f7fu64; // All bits except the H file (right edge)
    let file = 0xffffffffffffffffu64;

    match dir {
        Direction::East => (1, not_h_file),
        Direction::West => (-1, not_a_file),
        Direction::South => (8, file),
        Direction::North => (-8, file),
        Direction::SouthEast => (9, not_h_file),
        Direction::NorthWest => (7, not_a_file),
        Direction::SouthWest => (-7, not_h_file),
        Direction::NorthEast => (-9, not_a_file),
    }
}

fn shift_bits(bits: u64, shift: i32) -> u64 {
    if shift > 0 {
        (bits << shift)
    } else {
        (bits >> (-shift))
    }
}

fn get_valid_moves_bits(player_bits: u64, opponent_bits: u64) -> u64 {
    let empty = !(player_bits | opponent_bits);
    let mut valid_moves = 0u64;

    for dir in Direction::DIRECTIONS {
        let (shift, mask) = get_shift_and_mask(dir);
        let mut tmp = shift_bits(player_bits, shift) & opponent_bits & mask;
        while tmp != 0 {
            let next = shift_bits(tmp, shift) & mask;
            if next & empty != 0 {
                valid_moves |= next & empty;
                break;
            }
            if next & player_bits != 0 {
                break;
            }
            tmp = next & opponent_bits;
        }
    }

    valid_moves
}

fn get_flips_bits(move_bit: u64, player_bits: u64, opponent_bits: u64) -> u64 {
    let empty = !(player_bits | opponent_bits);
    let mut flips = 0u64;

    for dir in Direction::DIRECTIONS {
        let (shift, mask) = get_shift_and_mask(dir);
        let mut tmp = shift_bits(move_bit, shift) & opponent_bits & mask;
        while tmp != 0 {
            let next = shift_bits(tmp, shift) & mask;
            if next & empty != 0 {
                break;
            }
            if next & player_bits != 0 {
                flips |= tmp;
                break;
            }
            tmp = next & opponent_bits;
        }
    }

    flips
}

impl BitBoard {
    pub fn new() -> Self {
        let mut board = BitBoard { black: 0, white: 0 };
        board.set_disc(&Position::D4, Color::White);
        board.set_disc(&Position::E5, Color::White);
        board.set_disc(&Position::E4, Color::Black);
        board.set_disc(&Position::D5, Color::Black);

        board
    }

    pub fn get_disc(&self, pos: &Position) -> Option<Color> {
        let index = pos.y * BOARD_SIZE as i32 + pos.x;
        let bit = 1u64 << index;
        if self.black & bit != 0 {
            Some(Color::Black)
        } else if self.white & bit != 0 {
            Some(Color::White)
        } else {
            None
        }
    }

    pub fn set_disc(&mut self, pos: &Position, color: Color) {
        let index = pos.y * BOARD_SIZE as i32 + pos.x;
        let bit = 1u64 << index;
        match color {
            Color::Black => {
                self.black |= bit;
                self.white &= !bit;
            }
            Color::White => {
                self.white |= bit;
                self.black &= !bit;
            }
        }
    }
}

impl Board for BitBoard {
    fn discs(&self) -> Vec<Vec<Option<Color>>> {
        let mut discs: Vec<Vec<Option<Color>>> = Vec::new();

        for y in 0..8 {
            let mut row: Vec<Option<Color>> = Vec::new();
            for x in 0..8 {
                row.push(self.get_disc(&Position { x, y }));
            }
            discs.push(row);
        }

        discs
    }

    fn count_of(&self, color: Option<Color>) -> usize {
        match color {
            Some(c) => match c {
                Color::Black => self.black.count_ones() as usize,
                Color::White => self.white.count_ones() as usize,
            },
            None => (64 - self.black.count_ones() - self.white.count_ones()) as usize,
        }
    }

    fn make_move(&mut self, mov: &Move) -> bool {
        if mov.position.is_none() {
            return false;
        }

        let pos = mov.position.unwrap();
        let idx = pos.x + pos.y * BOARD_SIZE as i32;
        let move_bit = 1u64 << idx;

        let (player_bits, opponent_bits) = match mov.color {
            Color::Black => (&mut self.black, &mut self.white),
            Color::White => (&mut self.white, &mut self.black),
        };
        let valid_moves = get_valid_moves_bits(*player_bits, *opponent_bits);

        if valid_moves & move_bit == 0 {
            // Invalid move
            return false;
        }

        let flips = get_flips_bits(move_bit, *player_bits, *opponent_bits);

        *player_bits |= move_bit | flips;
        *opponent_bits &= !flips;

        true
    }

    fn get_valid_moves(&self, color: Color) -> Vec<Position> {
        let (player_bits, opponent_bits) = match color {
            Color::Black => (self.black, self.white),
            Color::White => (self.white, self.black),
        };
        let mut bits = get_valid_moves_bits(player_bits, opponent_bits);
        let mut positions = Vec::new();
        while bits != 0 {
            let lsb = bits & (!bits + 1); // 最下位の1ビットを取得
            let index = lsb.trailing_zeros() as usize;
            let x = (index % 8) as i32;
            let y = (index / 8) as i32;
            positions.push(Position { x, y });
            bits &= bits - 1; // 最下位の1ビットをクリア
        }
        positions
    }

    fn display(&self) {
        println!("  A B C D E F G H");
        for y in 0..8 {
            print!("{}", y + 1);
            for x in 0..8 {
                print!(" ");
                match self.get_disc(&Position { x, y }) {
                    Some(Color::Black) => print!("B"), // 黒の駒
                    Some(Color::White) => print!("W"), // 白の駒
                    None => print!("-"),               // 駒なし
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_valid_moves() {
        let board = BitBoard::new();
        let valid_moves = board.get_valid_moves(Color::Black);
        let expected_moves = vec![Position::D3, Position::C4, Position::F5, Position::E6];
        assert_eq!(valid_moves, expected_moves);
    }

    #[test]
    fn test_make_move() {
        let mut board = BitBoard::new();

        let mov = Move {
            color: Color::Black,
            position: Some(Position::D3),
        };
        let r = board.make_move(&mov);
        assert!(r);
        board.display();
        let expected_black = 0x0000000818080000u64;
        let expected_white = 0x0000001000000000u64;
        assert_eq!(board.black, expected_black);
        assert_eq!(board.white, expected_white);
    }
}
