use crate::{
    board::{Board, BOARD_SIZE},
    CellState, Color, Direction, Position,
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
    pub fn init_board() -> Self {
        let mut board = Self::default();
        board.init();
        board
    }

    fn is_valid_move(&self, color: Color, pos: &Position) -> bool {
        if self.discs[pos.x as usize + pos.y as usize * BOARD_SIZE] != EMPTY {
            return false;
        }

        let opponent = get_color_value(Some(color.opponent()));

        for dir in Direction::DIRECTIONS {
            let (dx, dy) = get_direction_vector(dir);
            let mut x = pos.x as i8 + dx;
            let mut y = pos.y as i8 + dy;
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
    fn cell_states(&self) -> [CellState; BOARD_SIZE * BOARD_SIZE] {
        let mut cells = [CellState::Empty; BOARD_SIZE * BOARD_SIZE];
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                let index = x + y * BOARD_SIZE;
                cells[index] = match self.discs[index] {
                    EMPTY => CellState::Empty,
                    BLACK => CellState::Disc(Color::Black),
                    WHITE => CellState::Disc(Color::White),
                    _ => CellState::Empty,
                };
            }
        }

        cells
    }

    fn get_cell_state(&self, pos: &Position) -> CellState {
        let index = pos.x as usize + pos.y as usize * BOARD_SIZE;
        match self.discs[index] {
            EMPTY => CellState::Empty,
            BLACK => CellState::Disc(Color::Black),
            WHITE => CellState::Disc(Color::White),
            _ => CellState::Empty,
        }
    }

    fn set_cell_state(&mut self, pos: &Position, cell_state: CellState) {
        let s = match cell_state {
            CellState::Empty => EMPTY,
            CellState::Disc(Color::Black) => BLACK,
            CellState::Disc(Color::White) => WHITE,
        };
        self.discs[pos.to_index()] = s;
    }

    fn count_of(&self, cell_state: CellState) -> usize {
        let c = match cell_state {
            CellState::Empty => EMPTY,
            CellState::Disc(Color::Black) => BLACK,
            CellState::Disc(Color::White) => WHITE,
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
            let mut x = pos.x as i8 + dx;
            let mut y = pos.y as i8 + dy;
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
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                let pos = Position::new(x, y);
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
        let board = ArrayBoard::init_board();

        assert!(board.is_valid_move(Color::Black, &Position::C4));
        assert!(!board.is_valid_move(Color::Black, &Position::A1));
    }

    #[test]
    fn test_make_move() {
        let mut board = ArrayBoard::init_board();

        assert!(board.make_move(Color::Black, &Position::C4));
        assert_eq!(board.discs[2 + 3 * BOARD_SIZE], BLACK);
        assert_eq!(board.discs[3 + 3 * BOARD_SIZE], BLACK);
        assert_eq!(board.discs[4 + 3 * BOARD_SIZE], BLACK);
    }

    #[test]
    fn test_count_of() {
        let board = ArrayBoard::init_board();

        assert_eq!(board.count_of(CellState::Disc(Color::Black)), 2);
        assert_eq!(board.count_of(CellState::Disc(Color::White)), 2);
        assert_eq!(
            board.count_of(CellState::Empty),
            BOARD_SIZE * BOARD_SIZE - 4
        );
    }

    #[test]
    fn test_get_valid_moves() {
        let board = ArrayBoard::init_board();

        let valid_move_pos = board.get_valid_moves(Color::Black);
        println!("valid_moves: {:?}", valid_move_pos);
        assert!(valid_move_pos.contains(&Position::C4));
        assert!(valid_move_pos.contains(&Position::D3));
        assert!(valid_move_pos.contains(&Position::E6));
        assert!(valid_move_pos.contains(&Position::F5));
    }

    #[test]
    fn test_display() {
        let board = ArrayBoard::init_board();

        board.display();
    }
}
