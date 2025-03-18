use temp_game_ai::GameState;
use temp_reversi_core::{Bitboard, Player, Position};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReversiState {
    pub board: Bitboard,
    pub player: Player,
}

impl GameState for ReversiState {
    type Move = Position;

    fn is_terminal(&self) -> bool {
        self.board.valid_moves(self.player).is_empty()
            && self.board.valid_moves(self.player.opponent()).is_empty()
    }

    fn generate_children(&self) -> Vec<(Self, Self::Move)> {
        self.board
            .valid_moves(self.player)
            .iter()
            .map(|&pos| {
                let mut board = self.board.clone();
                board.apply_move(pos, self.player).unwrap();
                (
                    ReversiState {
                        board,
                        player: self.player.opponent(),
                    },
                    pos,
                )
            })
            .collect()
    }
}

mod mobility;
mod pattern;
mod phase_aware;
mod positional;
mod simple;
mod tempura;

pub use mobility::*;
pub use pattern::*;
pub use phase_aware::*;
pub use positional::*;
pub use simple::*;
pub use tempura::*;
