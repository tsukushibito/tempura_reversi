use std::hash::Hash;
use temp_game_ai::GameState;
use temp_reversi_core::{Bitboard, Player, Position};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReversiState {
    pub board: Bitboard,
    pub player: Player,
}

impl Hash for ReversiState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.board.hash(state);
        self.player.hash(state);
    }
}

impl GameState for ReversiState {
    type Move = Position;

    fn generate_children(&self) -> Vec<(Self, Self::Move)> {
        let valid_moves = self.board.valid_moves(self.player);

        // if self.passed && valid_moves.len() == 1 && valid_moves[0] == Position::PASS {
        //     return vec![];
        // }

        valid_moves
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
