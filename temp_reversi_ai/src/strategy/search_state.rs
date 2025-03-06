use std::hash::{Hash, Hasher};

use temp_reversi_core::{Bitboard, Player, Position};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct SearchState {
    pub board: Bitboard,
    pub current_player: Player,
}

const FNV_OFFSET: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

/// Hashes a Player to a u64 using FNV hash parameters.
fn hash_player(player: Player) -> u64 {
    let mut hash = FNV_OFFSET;
    let player_byte: u8 = match player {
        Player::Black => 0,
        Player::White => 1,
    };
    hash ^= player_byte as u64;
    hash = hash.wrapping_mul(FNV_PRIME);
    hash
}

impl SearchState {
    pub fn new(board: Bitboard, current_player: Player) -> Self {
        Self {
            board,
            current_player,
        }
    }

    pub fn apply_move(&self, pos: Position) -> Option<SearchState> {
        let mut new_board = self.board.clone();
        if new_board.apply_move(pos, self.current_player).is_ok() {
            Some(SearchState {
                board: new_board,
                current_player: self.current_player.opponent(),
            })
        } else {
            None
        }
    }
}

impl Hash for SearchState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.board.get_hash().hash(state);
        let player_hash = hash_player(self.current_player);
        player_hash.hash(state);
    }
}
