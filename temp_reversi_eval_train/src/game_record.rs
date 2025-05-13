use serde::{Deserialize, Serialize};
use temp_reversi_core::{Game, Position};

use crate::dataset::ReversiSample;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GameRecord {
    /// Sequence of moves represented as board indices (0-63).
    pub moves: Vec<u8>,
    /// Final score of the game, represented as (black, white).
    pub final_score: (u8, u8),
}

impl GameRecord {
    pub fn to_samples(&self) -> Vec<ReversiSample> {
        let mut game = Game::default();
        let mut samples = Vec::new();

        for m in &self.moves {
            let pos = Position::from_u8(*m);
            let _ = game.apply_move(pos);
            let board = game.board_state();
            // let feature = extract_feature(board);
            // let packed_feature = FEATURE_PACKER.pack(&feature);
            let stone_diff = self.final_score.0 as i8 - self.final_score.1 as i8;
            // let sample = ReversiSample {
            //     indices: packed_feature.indices.to_vec(),
            //     phase: packed_feature.phase,
            //     stone_diff: label,
            // };
            let (black_bits, white_bits) = board.bits();
            let sample = ReversiSample {
                black_bits,
                white_bits,
                stone_diff,
            };
            samples.push(sample);
        }

        samples
    }
}

#[cfg(test)]
mod tests {}
