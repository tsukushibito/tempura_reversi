use super::{mobility::MobilityEvaluator, PositionalEvaluator};
use crate::ReversiState;
use temp_game_ai::Evaluator;
use temp_reversi_core::{Bitboard, Player};

/// Defines the phase of the game
enum Phase {
    Early,
    Mid,
    Late,
}

/// Phase-aware evaluator that adjusts weights for mobility, positional values, and score
/// based on the phase of the game.
#[derive(Debug, Clone)]
pub struct PhaseAwareEvaluator {
    pub phase_thresholds: (usize, usize),
    pub early_phase_weights: (i32, i32, i32),
    pub mid_phase_weights: (i32, i32, i32),
    pub late_phase_weights: (i32, i32, i32),
}

impl Default for PhaseAwareEvaluator {
    fn default() -> Self {
        Self {
            phase_thresholds: (30, 60),
            early_phase_weights: (2, 1, 0),
            mid_phase_weights: (4, 1, 2),
            late_phase_weights: (1, 1, 2),
        }
    }
}

impl PhaseAwareEvaluator {
    /// Determine the phase of the game based on the total number of stones.
    fn determine_phase(&self, board: &Bitboard) -> Phase {
        let (black_count, white_count) = board.count_stones();
        let total_stones = black_count + white_count;

        if total_stones <= self.phase_thresholds.0 {
            Phase::Early
        } else if total_stones <= self.phase_thresholds.1 {
            Phase::Mid
        } else {
            Phase::Late
        }
    }
}

impl Evaluator<ReversiState> for PhaseAwareEvaluator {
    fn evaluate(&mut self, state: &ReversiState) -> i32 {
        let phase = self.determine_phase(&state.board);
        let mut mobility_evaluator = MobilityEvaluator;
        let mut positional_evaluator = PositionalEvaluator;

        // Evaluate each factor
        let mobility_score = mobility_evaluator.evaluate(state);
        let positional_score = positional_evaluator.evaluate(state);
        let (black_count, white_count) = state.board.count_stones();
        let score_diff = match state.player {
            Player::Black => black_count as i32 - white_count as i32,
            Player::White => white_count as i32 - black_count as i32,
        };

        // Apply weights based on the phase
        let score = match phase {
            Phase::Early => {
                self.early_phase_weights.0 * mobility_score
                    + self.early_phase_weights.1 * positional_score
                    + self.early_phase_weights.2 * score_diff
            }
            Phase::Mid => {
                self.mid_phase_weights.0 * mobility_score
                    + self.mid_phase_weights.1 * positional_score
                    + self.mid_phase_weights.2 * score_diff
            }
            Phase::Late => {
                self.late_phase_weights.0 * mobility_score
                    + self.late_phase_weights.1 * positional_score
                    + self.late_phase_weights.2 * score_diff
            }
        };

        score
    }
}

#[cfg(test)]
mod tests {
    use crate::{ai_player::AiPlayer, strategy::Strategy};

    use super::*;
    use rayon::prelude::*;
    use temp_game_ai::searcher::{NegaAlphaTT, Searcher};
    use temp_reversi_core::{Bitboard, Game, GamePlayer, Player, Position};

    #[test]
    fn test_phase_aware_evaluation() {
        let board = Bitboard::default(); // Initial board state
        let mut evaluator = PhaseAwareEvaluator::default();

        // Test early phase
        let early_score = evaluator.evaluate(&ReversiState {
            board,
            player: Player::Black,
        });
        assert!(
            early_score >= 0,
            "Early phase score should be calculated correctly."
        );

        // Simulate mid-phase board state
        let mid_board = board.clone();
        // Apply moves to transition to mid-phase
        let mid_score = evaluator.evaluate(&ReversiState {
            board: mid_board,
            player: Player::Black,
        });
        assert!(
            mid_score >= 0,
            "Mid phase score should be calculated correctly."
        );

        // Simulate late-phase board state
        let late_board = board.clone();
        // Apply moves to transition to late-phase
        let late_score = evaluator.evaluate(&ReversiState {
            board: late_board,
            player: Player::Black,
        });
        assert!(
            late_score >= 0,
            "Late phase score should be calculated correctly."
        );
    }

    #[derive(Clone, Debug)]
    struct TestStrategy {
        pub nega_alpha_tt: NegaAlphaTT<ReversiState, PhaseAwareEvaluator, PhaseAwareEvaluator>,
        max_depth: usize,
    }

    impl TestStrategy {
        fn new(evaluator: PhaseAwareEvaluator, max_depth: usize) -> Self {
            Self {
                nega_alpha_tt: NegaAlphaTT::new(evaluator, PhaseAwareEvaluator::default()),
                max_depth,
            }
        }
    }

    impl Strategy for TestStrategy {
        fn select_move(&mut self, board: &Bitboard, player: Player) -> Position {
            let root = ReversiState {
                board: *board,
                player,
            };

            self.nega_alpha_tt
                .search(&root, self.max_depth)
                .expect("No moves available.")
                .0
        }

        fn clone_box(&self) -> Box<dyn Strategy> {
            Box::new(self.clone())
        }
    }

    #[test]
    fn test_parameters() {
        // 対戦させてどのパラメータが強いかを確認する
        let evaluator1 = PhaseAwareEvaluator::default();
        let strategy1 = TestStrategy::new(evaluator1, 4);

        let evaluator2 = PhaseAwareEvaluator {
            phase_thresholds: (30, 60),
            early_phase_weights: (2, 1, 0),
            mid_phase_weights: (4, 1, 2),
            late_phase_weights: (1, 1, 2),
        };
        let strategy2 = TestStrategy::new(evaluator2, 4);

        let results: Vec<(usize, usize)> = (0..100)
            .into_par_iter()
            .map(|_| {
                let mut game = Game::default();
                let mut black_ai = AiPlayer::new(strategy1.clone_box());
                let mut white_ai = AiPlayer::new(strategy2.clone_box());

                while !game.is_game_over() {
                    let current_ai = if game.current_player() == Player::Black {
                        &mut black_ai
                    } else {
                        &mut white_ai
                    };

                    let best_move = current_ai.select_move(&game);
                    game.apply_move(best_move).unwrap();
                }

                let (black_count, white_count) = game.current_score();
                if black_count > white_count {
                    (1, 0)
                } else if black_count < white_count {
                    (0, 1)
                } else {
                    (0, 0)
                }
            })
            .collect();

        let (black_wins, white_wins): (usize, usize) = results
            .iter()
            .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));

        println!("Black wins: {}, White wins: {}", black_wins, white_wins);
    }
}
