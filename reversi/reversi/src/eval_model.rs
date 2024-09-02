use std::path::Path;

use crate::{
    Ai, BitBoard, Config, Game, Negaalpha, ResultBoxErr, Searcher, TempuraEvaluator, TestEvaluator,
};

pub fn eval_model<P: AsRef<Path>>(config: P) -> ResultBoxErr<()> {
    let config = Config::from_file(config)?;
    let model_path = config.training_models_path();
    let evaluator = TempuraEvaluator::load(model_path)?;

    let mut game = Game::initial();

    let mut ai = Ai {
        searcher: Searcher::TempuraNegaalpha(Negaalpha::new(evaluator)),
        search_depth: 4,
    };

    let mut test_ai = Ai {
        searcher: Searcher::TestNegaalpha(Negaalpha::new(TestEvaluator::default())),
        search_depth: 4,
    };

    let mut scores: Vec<(usize, usize)> = Default::default();
    let mut black_wins = 0;
    let mut white_wins = 0;
    let mut draw = 0;

    for _ in 0..100 {
        let mut black_ai = ai;
        let mut white_ai = test_ai;

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

        scores.push((game.black_score(), game.white_score()));
        println!("b:{}, w:{}", game.black_score(), game.white_score());
        match game.black_score().cmp(&game.white_score()) {
            std::cmp::Ordering::Less => white_wins += 1,
            std::cmp::Ordering::Equal => draw += 1,
            std::cmp::Ordering::Greater => black_wins += 1,
        }

        ai = black_ai;
        test_ai = white_ai;

        game.reset();
    }

    println!("b:{black_wins}, w:{white_wins}, d:{draw}");
    println!("{:?}", scores);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Config;

    use super::*;

    #[test]
    fn test_eval_model() -> ResultBoxErr<()> {
        let config = "test_config.json";
        let config = Config::from_file(config)?;

        eval_model(config.training_models_path())?;

        Ok(())
    }
}
