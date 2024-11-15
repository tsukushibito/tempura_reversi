use crate::{
    board::{Board, BOARD_SIZE},
    Color, Direction, Position,
};

const EMPTY: u8 = 0;
const BLACK: u8 = 1;
const WHITE: u8 = 2;

fn get_color_value(color: Option<Color>) -> u8 {
    match color {
        None => EMPTY,
        Some(Color::Black) => BLACK,
        Some(Color::White) => WHITE,
    }
}

fn get_direction_vector(dir: Direction) -> (i8, i8) {
    match dir {
        Direction::East => (0, 1),
        Direction::West => (0, -1),
        Direction::South => (1, 0),
        Direction::North => (-1, 0),
        Direction::SouthEast => (1, 1),
        Direction::NorthWest => (-1, -1),
        Direction::SouthWest => (1, -1),
        Direction::NorthEast => (-1, 1),
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ArrayBoard {
    pub discs: [u8; BOARD_SIZE * BOARD_SIZE],
}

impl ArrayBoard {
    fn is_valid_move(&self, color: Color, pos: &Position) -> bool {
        if self.discs[pos.x as usize + pos.y as usize * BOARD_SIZE] != EMPTY {
            return false;
        }

        let opponent = get_color_value(Some(color.opponent()));

        for dir in Direction::DIRECTIONS {
            let (dx, dy) = get_direction_vector(dir);
            let mut x = pos.x + dx;
            let mut y = pos.y + dy;
            let mut found_opponent = false;

            while x >= 0 && x < BOARD_SIZE as i8 && y >= 0 && y < BOARD_SIZE as i8 {
                let index = x as usize + y as usize * BOARD_SIZE;
                match self.discs[index] {
                    d if d == opponent => found_opponent = true,
                    d if d == color as u8 && found_opponent => return true,
                    _ => break,
                }
                x += dx;
                y += dy;
            }
        }

        false
    }
}

impl Default for ArrayBoard {
    fn default() -> Self {
        ArrayBoard {
            discs: [EMPTY; BOARD_SIZE * BOARD_SIZE],
        }
    }
}

impl Board for ArrayBoard {
    fn discs(&self) -> Vec<Vec<Option<Color>>> {
        let mut discs = Vec::new();
        for y in 0..BOARD_SIZE {
            let mut row = Vec::new();
            for x in 0..BOARD_SIZE {
                let index = x + y * BOARD_SIZE;
                let color = match self.discs[index] {
                    EMPTY => None,
                    BLACK => Some(Color::Black),
                    WHITE => Some(Color::White),
                    _ => None,
                };
                row.push(color);
            }
            discs.push(row);
        }

        discs
    }

    fn get_disc(&self, pos: &Position) -> Option<Color> {
        let index = pos.x as usize + pos.y as usize * BOARD_SIZE;
        match self.discs[index] {
            EMPTY => None,
            BLACK => Some(Color::Black),
            WHITE => Some(Color::White),
            _ => None,
        }
    }

    fn set_disc(&mut self, pos: &Position, color: Option<Color>) {
        let c = match color {
            None => EMPTY,
            Some(col) => match col {
                Color::Black => BLACK,
                Color::White => WHITE,
            },
        };
        let index = pos.x as usize + pos.y as usize * BOARD_SIZE;
        self.discs[index] = c;
    }

    fn count_of(&self, color: Option<Color>) -> usize {
        let c = match color {
            None => EMPTY,
            Some(col) => match col {
                Color::Black => BLACK,
                Color::White => WHITE,
            },
        };
        let mut count = 0;
        for disc in self.discs {
            if disc == c {
                count += 1;
            }
        }
        count
    }

    fn make_move(&mut self, color: Color, pos: &Position) -> bool {
        if !self.is_valid_move(color, pos) {
            return false;
        }

        let player = get_color_value(Some(color));
        let opponent = get_color_value(Some(color.opponent()));

        let mut to_flip = Vec::new();

        for dir in Direction::DIRECTIONS {
            let (dx, dy) = get_direction_vector(dir);
            let mut x = pos.x + dx;
            let mut y = pos.y + dy;
            let mut potential_flips = Vec::new();

            while x >= 0 && x < BOARD_SIZE as i8 && y >= 0 && y < BOARD_SIZE as i8 {
                let index = x as usize + y as usize * BOARD_SIZE;
                match self.discs[index] {
                    d if d == opponent => potential_flips.push((x, y)),
                    d if d == player => {
                        to_flip.extend(potential_flips);
                        break;
                    }
                    _ => break,
                }
                x += dx;
                y += dy;
            }
        }

        for (x, y) in to_flip {
            let index = x as usize + y as usize * BOARD_SIZE;
            self.discs[index] = color as u8;
        }

        let index = pos.x as usize + pos.y as usize * BOARD_SIZE;
        self.discs[index] = color as u8;

        true
    }

    fn get_valid_moves(&self, color: Color) -> Vec<Position> {
        let mut valid_moves = Vec::new();
        for y in 0..BOARD_SIZE as i8 {
            for x in 0..BOARD_SIZE as i8 {
                let pos = Position { x, y };
                if self.is_valid_move(color, &pos) {
                    valid_moves.push(pos);
                }
            }
        }
        valid_moves
    }

    fn display(&self) {
        println!("  A B C D E F G H");
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                print!(" ");
                let index = x + y * BOARD_SIZE;
                match self.discs[index] {
                    EMPTY => print!("-"),
                    BLACK => print!("B"),
                    WHITE => print!("W"),
                    _ => print!("?"),
                };
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_move() {
        let board = ArrayBoard::new();

        assert!(board.is_valid_move(Color::Black, &Position::C4));
        assert!(!board.is_valid_move(Color::Black, &Position::A1));
    }

    #[test]
    fn test_make_move() {
        let mut board = ArrayBoard::new();

        assert!(board.make_move(Color::Black, &Position::C4));
        assert_eq!(board.discs[2 + 3 * BOARD_SIZE], BLACK);
        assert_eq!(board.discs[3 + 3 * BOARD_SIZE], BLACK);
        assert_eq!(board.discs[4 + 3 * BOARD_SIZE], BLACK);
    }

    #[test]
    fn test_count_of() {
        let board = ArrayBoard::new();

        assert_eq!(board.count_of(Some(Color::Black)), 2);
        assert_eq!(board.count_of(Some(Color::White)), 2);
        assert_eq!(board.count_of(None), BOARD_SIZE * BOARD_SIZE - 4);
    }

    #[test]
    fn test_get_valid_moves() {
        let board = ArrayBoard::new();

        let valid_move_pos = board.get_valid_moves(Color::Black);
        println!("valid_moves: {:?}", valid_move_pos);
        assert!(valid_move_pos.contains(&Position::C4));
        assert!(valid_move_pos.contains(&Position::D3));
        assert!(valid_move_pos.contains(&Position::E6));
        assert!(valid_move_pos.contains(&Position::F5));
    }

    #[test]
    fn test_display() {
        let board = ArrayBoard::new();

        board.display();
    }
}
