use board::BOARD_SIZE;

pub mod ai;
pub mod array_board;
pub mod bit_board;
pub mod board;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i8,
    pub y: i8,
}

impl Position {
    pub const A1: Position = Position { x: 0, y: 0 };
    pub const A2: Position = Position { x: 0, y: 1 };
    pub const A3: Position = Position { x: 0, y: 2 };
    pub const A4: Position = Position { x: 0, y: 3 };
    pub const A5: Position = Position { x: 0, y: 4 };
    pub const A6: Position = Position { x: 0, y: 5 };
    pub const A7: Position = Position { x: 0, y: 6 };
    pub const A8: Position = Position { x: 0, y: 7 };
    pub const B1: Position = Position { x: 1, y: 0 };
    pub const B2: Position = Position { x: 1, y: 1 };
    pub const B3: Position = Position { x: 1, y: 2 };
    pub const B4: Position = Position { x: 1, y: 3 };
    pub const B5: Position = Position { x: 1, y: 4 };
    pub const B6: Position = Position { x: 1, y: 5 };
    pub const B7: Position = Position { x: 1, y: 6 };
    pub const B8: Position = Position { x: 1, y: 7 };
    pub const C1: Position = Position { x: 2, y: 0 };
    pub const C2: Position = Position { x: 2, y: 1 };
    pub const C3: Position = Position { x: 2, y: 2 };
    pub const C4: Position = Position { x: 2, y: 3 };
    pub const C5: Position = Position { x: 2, y: 4 };
    pub const C6: Position = Position { x: 2, y: 5 };
    pub const C7: Position = Position { x: 2, y: 6 };
    pub const C8: Position = Position { x: 2, y: 7 };
    pub const D1: Position = Position { x: 3, y: 0 };
    pub const D2: Position = Position { x: 3, y: 1 };
    pub const D3: Position = Position { x: 3, y: 2 };
    pub const D4: Position = Position { x: 3, y: 3 };
    pub const D5: Position = Position { x: 3, y: 4 };
    pub const D6: Position = Position { x: 3, y: 5 };
    pub const D7: Position = Position { x: 3, y: 6 };
    pub const D8: Position = Position { x: 3, y: 7 };
    pub const E1: Position = Position { x: 4, y: 0 };
    pub const E2: Position = Position { x: 4, y: 1 };
    pub const E3: Position = Position { x: 4, y: 2 };
    pub const E4: Position = Position { x: 4, y: 3 };
    pub const E5: Position = Position { x: 4, y: 4 };
    pub const E6: Position = Position { x: 4, y: 5 };
    pub const E7: Position = Position { x: 4, y: 6 };
    pub const E8: Position = Position { x: 4, y: 7 };
    pub const F1: Position = Position { x: 5, y: 0 };
    pub const F2: Position = Position { x: 5, y: 1 };
    pub const F3: Position = Position { x: 5, y: 2 };
    pub const F4: Position = Position { x: 5, y: 3 };
    pub const F5: Position = Position { x: 5, y: 4 };
    pub const F6: Position = Position { x: 5, y: 5 };
    pub const F7: Position = Position { x: 5, y: 6 };
    pub const F8: Position = Position { x: 5, y: 7 };
    pub const G1: Position = Position { x: 6, y: 0 };
    pub const G2: Position = Position { x: 6, y: 1 };
    pub const G3: Position = Position { x: 6, y: 2 };
    pub const G4: Position = Position { x: 6, y: 3 };
    pub const G5: Position = Position { x: 6, y: 4 };
    pub const G6: Position = Position { x: 6, y: 5 };
    pub const G7: Position = Position { x: 6, y: 6 };
    pub const G8: Position = Position { x: 6, y: 7 };
    pub const H1: Position = Position { x: 7, y: 0 };
    pub const H2: Position = Position { x: 7, y: 1 };
    pub const H3: Position = Position { x: 7, y: 2 };
    pub const H4: Position = Position { x: 7, y: 3 };
    pub const H5: Position = Position { x: 7, y: 4 };
    pub const H6: Position = Position { x: 7, y: 5 };
    pub const H7: Position = Position { x: 7, y: 6 };
    pub const H8: Position = Position { x: 7, y: 7 };

    pub fn from_index(index: i8) -> Self {
        Position {
            x: index % BOARD_SIZE as i8,
            y: index / BOARD_SIZE as i8,
        }
    }

    pub fn to_index(&self) -> i8 {
        self.y * BOARD_SIZE as i8 + self.x
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black = 1,
    White = 2,
}

impl Color {
    pub fn opponent(&self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub position: Option<Position>,
    pub color: Color,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    East,      // 東
    West,      // 西
    South,     // 南
    North,     // 北
    SouthEast, // 南東
    SouthWest, // 南西
    NorthEast, // 北東
    NorthWest, // 北西
}

impl Direction {
    pub const DIRECTIONS: [Direction; 8] = [
        Direction::East,
        Direction::West,
        Direction::South,
        Direction::North,
        Direction::SouthEast,
        Direction::NorthWest,
        Direction::SouthWest,
        Direction::NorthEast,
    ];

    pub fn dx(&self) -> i32 {
        match self {
            Direction::East => 1,
            Direction::West => -1,
            Direction::South => 0,
            Direction::North => 0,
            Direction::SouthEast => 1,
            Direction::NorthWest => -1,
            Direction::SouthWest => -1,
            Direction::NorthEast => 1,
        }
    }

    pub fn dy(&self) -> i32 {
        match self {
            Direction::East => 0,
            Direction::West => 0,
            Direction::South => 1,
            Direction::North => -1,
            Direction::SouthEast => 1,
            Direction::NorthWest => -1,
            Direction::SouthWest => 1,
            Direction::NorthEast => -1,
        }
    }
}

#[cfg(test)]
mod tests {}