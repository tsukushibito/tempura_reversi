use temp_reversi_core::Game;
use temp_reversi_eval::feature::extract_feature;

#[derive(Debug, Clone, Default)]
pub struct DataSample {
    pub feature: Vec<u8>,
    pub label: f32,
}

#[derive(Debug, Clone, Default)]
pub struct Dataset {
    pub samples: Vec<DataSample>,
}

#[derive(Debug, Clone, Default)]
pub struct GameRecord {
    /// Sequence of moves represented as board indices (0-63).
    pub moves: Vec<u8>,
    /// Final score of the game, represented as (black, white).
    pub final_score: (u8, u8),
}

impl GameRecord {
    pub fn to_dataset(&self) -> Dataset {
        let mut dataset = Dataset::default();
        let mut game = Game::default(); // Placeholder for actual game state initialization

        for &move_ in &self.moves {
            let (black, white) = game.board_state().count_stones();
            let phase = (black + white) as u8;
            let feature = extract_feature(&game.board_state());
        }
        dataset
    }
}
