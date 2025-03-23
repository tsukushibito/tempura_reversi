use std::hash::Hash;
use temp_game_ai::GameState;
use temp_reversi_core::{Bitboard, Player, Position};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReversiState2 {
    pub board: Bitboard,
    pub player: Player,
    undo_stack: Vec<Bitboard>,
    redo_stack: Vec<Bitboard>,
}

impl Hash for ReversiState2 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.board.hash(state);
        self.player.hash(state);
    }
}

impl GameState for ReversiState2 {
    type Move = Position;

    fn generate_children(&self) -> Vec<(Self, Self::Move)> {
        let valid_moves = self.board.valid_moves(self.player);

        valid_moves
            .iter()
            .map(|&pos| {
                let mut board = self.board.clone();
                board.apply_move(pos, self.player).unwrap();
                (
                    ReversiState2 {
                        board,
                        player: self.player.opponent(),
                        undo_stack: vec![],
                        redo_stack: vec![],
                    },
                    pos,
                )
            })
            .collect()
    }

    fn valid_moves(&self) -> Vec<Self::Move> {
        self.board.valid_moves(self.player)
    }

    fn make_move(&mut self, mv: &Self::Move) {
        self.undo_stack.push(self.board.clone());
        self.board.apply_move(*mv, self.player).unwrap();
        self.player = self.player.opponent();
        self.redo_stack.clear();
    }

    fn undo_move(&mut self) {
        self.redo_stack.push(self.board.clone());
        self.board = self.undo_stack.pop().unwrap();
        self.player = self.player.opponent();
    }
}
