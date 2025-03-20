#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Player {
    #[default]
    Black,
    White,
}

impl Player {
    pub fn opponent(&self) -> Player {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}
