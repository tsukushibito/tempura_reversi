use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

use crate::{Ai, BitBoard, Game, Negaalpha, Searcher, TestEvaluator};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Winner {
    Black,
    White,
    #[default]
    Draw,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GameRecord {
    pub moves: Vec<u8>,
    pub winner: Winner,
    pub black_score: u8,
    pub white_score: u8,
}

#[derive(Debug)]
pub struct SelfPlaySetting {
    // black_ai_setting: AiSetting,
    // white_ai_setting: AiSetting,
    pub max_random_moves: usize,
    pub min_random_moves: usize,
}

pub fn self_play(setting: &SelfPlaySetting) -> GameRecord {
    let mut rng = rand::thread_rng();
    let mut game = Game::initial();
    let random_moves = rng.gen_range(setting.min_random_moves..setting.max_random_moves);
    for _ in 0..random_moves {
        if game.is_game_over() {
            break;
        }

        let current_player = game.current_player();
        let valid_moves = game.board().get_valid_moves(current_player);
        assert!(!valid_moves.is_empty());
        let pos = valid_moves.choose(&mut rng).unwrap();
        let _ = game.progress(current_player, *pos);
    }

    let mut black_ai = Ai {
        searcher: Searcher::TestNegaalpha(Negaalpha::new(TestEvaluator::default())),
        search_depth: 4,
    };

    let mut white_ai = Ai {
        searcher: Searcher::TestNegaalpha(Negaalpha::new(TestEvaluator::default())),
        search_depth: 4,
    };

    loop {
        if game.is_game_over() {
            break;
        }

        let bit_board = BitBoard::from_board(game.board());
        let ai = match game.current_player() {
            crate::Color::Black => &mut black_ai,
            crate::Color::White => &mut white_ai,
        };
        let mov = ai.decide_move(&bit_board, game.current_player());
        assert!(mov.is_some());

        if let Some(pos) = mov {
            let _ = game.progress(game.current_player(), pos);
        } else {
            break;
        }
    }

    let move_history = game.move_history();
    let moves: Vec<u8> = move_history
        .iter()
        .map(|m| m.position.to_index() as u8)
        .collect();
    let black_score = game.black_score() as u8;
    let white_score = game.white_score() as u8;
    let winner = match black_score.cmp(&white_score) {
        std::cmp::Ordering::Less => Winner::White,
        std::cmp::Ordering::Equal => Winner::Draw,
        std::cmp::Ordering::Greater => Winner::Black,
    };

    GameRecord {
        moves,
        winner,
        black_score,
        white_score,
    }
}
