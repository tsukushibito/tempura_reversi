use rand::rngs::StdRng;
use rand::{self, Rng, SeedableRng};

use crate::ai::evaluator::Evaluator;
use crate::ai::SearchResult;
use crate::bit_board::BitBoard;
use crate::board::{Board, BOARD_SIZE};
use crate::{Color, Move, Position};

// type EvalFunc = fn(&BitBoard, Color, f32) -> i32;

pub struct TranspositionTableEntry {
    pub score: i32,
    pub depth: u8,
    pub best_move: i8,
    pub policy: [i32; BOARD_SIZE * BOARD_SIZE],
}

pub struct Negaalpha<E: Evaluator> {
    evaluator: E,
    use_move_ordering: bool,
    rng: StdRng,
}

impl<E: Evaluator> Negaalpha<E> {
    pub fn new(evaluator: E) -> Self {
        Negaalpha {
            evaluator,
            use_move_ordering: true,
            rng: StdRng::from_entropy(),
        }
    }

    pub fn set_move_ordering(&mut self, enabled: bool) {
        self.use_move_ordering = enabled;
    }

    // fn evaluate_move(&self, _board: &BitBoard, _player: Color, pos: &Position) -> i32 {
    //     const POSITION_WEIGHTS: [[i32; 8]; 8] = [
    //         [100, -20, 10, 5, 5, 10, -20, 100],
    //         [-20, -50, -2, -2, -2, -2, -50, -20],
    //         [10, -2, -1, -1, -1, -1, -2, 10],
    //         [5, -2, -1, -1, -1, -1, -2, 5],
    //         [5, -2, -1, -1, -1, -1, -2, 5],
    //         [10, -2, -1, -1, -1, -1, -2, 10],
    //         [-20, -50, -2, -2, -2, -2, -50, -20],
    //         [100, -20, 10, 5, 5, 10, -20, 100],
    //     ];

    //     let x = pos.x as usize;
    //     let y = pos.y as usize;
    //     POSITION_WEIGHTS[y][x]
    // }

    // fn evaluate_move(&self, board: &BitBoard, player: Color, pos: &Position) -> i32 {
    //     let mut board = board.clone();
    //     board.make_move(player, pos);
    //     let my_moves = board.get_valid_moves(player).len() as i32;
    //     let opponent_moves = board.get_valid_moves(player.opponent()).len() as i32;
    //     my_moves - opponent_moves
    // }

    fn evaluate_move(&mut self, _board: &BitBoard, _player: Color, _pos: &Position) -> i32 {
        self.rng.gen()
    }

    pub fn search(
        &mut self,
        board: &BitBoard,
        player: Color,
        depth: u8,
        mut alpha: i32,
        beta: i32,
    ) -> SearchResult {
        let mut nodes_searched = 1;
        let mut policy = [0; BOARD_SIZE * BOARD_SIZE];

        let mut valid_moves = board.get_valid_moves(player);

        if depth == 0 || valid_moves.is_empty() {
            let score = self.evaluator.evaluate(board, player);
            return SearchResult {
                best_move: None,
                path: Vec::new(),
                nodes_searched,
                score,
                policy,
            };
        }

        if self.use_move_ordering {
            valid_moves.sort_by_cached_key(|pos| {
                let score = self.evaluate_move(board, player, pos);
                -score.checked_neg().unwrap_or(i32::MIN)
            });
        }

        let mut max_score = i32::MIN;
        let mut best_move = None;
        let mut best_path = Vec::new();

        for mv_pos in valid_moves {
            let mut new_board = board.clone();
            new_board.make_move(player, &mv_pos);

            let result = self.search(&new_board, player.opponent(), depth - 1, -beta, -alpha);

            let score = -result.score;

            nodes_searched += result.nodes_searched;

            let index = mv_pos.to_index();
            policy[index] = score;

            if score > max_score {
                max_score = score;
                best_move = Some(Move {
                    position: mv_pos,
                    color: player,
                });
                best_path = vec![Move {
                    position: mv_pos,
                    color: player,
                }];
                best_path.extend(result.path);
            }

            if score > alpha {
                alpha = score;
            }

            if alpha >= beta {
                break;
            }
        }

        let best_move_index = if let Some(bm) = best_move {
            bm.position.to_index() as i8
        } else {
            -1
        };

        SearchResult {
            best_move,
            path: best_path,
            nodes_searched,
            score: max_score,
            policy,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ai::evaluator::SimpleEvaluator, bit_board::BitBoard, Position};

    use super::*;

    #[test]
    fn test_negaalpha_no_move_ordering() {
        let bit_board = BitBoard::init_board();

        let mut negaalpha = Negaalpha::new(SimpleEvaluator::default());
        negaalpha.set_move_ordering(false);

        let depth = 9;

        let alpha = i32::MIN + 1;
        let beta = i32::MAX;

        let result = negaalpha.search(&bit_board, Color::Black, depth, alpha, beta);

        println!("best_move: {:?}", result.best_move);

        println!("path: ");
        let mut new_board = bit_board.clone();
        new_board.display();
        for mov in result.path {
            new_board.make_move(mov.color, &mov.position);
            new_board.display();
        }
        println!();

        let expected_best_move = Move {
            position: Position::D3,
            color: Color::Black,
        };

        assert_eq!(
            result.best_move,
            Some(expected_best_move),
            "ベストムーブが期待したものと異なります。"
        );

        println!("nodes_searched: {:?}", result.nodes_searched);
    }

    #[test]
    fn test_negaalpha_with_move_ordering() {
        let bit_board = BitBoard::init_board();

        let mut negaalpha = Negaalpha::new(SimpleEvaluator::default());
        negaalpha.set_move_ordering(true);

        let depth = 9;

        let alpha = i32::MIN + 1;
        let beta = i32::MAX;

        let result = negaalpha.search(&bit_board, Color::Black, depth, alpha, beta);

        println!("best_move: {:?}", result.best_move);

        println!("path: ");
        let mut new_board = bit_board.clone();
        new_board.display();
        for mov in result.path {
            new_board.make_move(mov.color, &mov.position);
            new_board.display();
        }
        println!();

        let expected_best_move = Move {
            position: Position::D3,
            color: Color::Black,
        };

        assert_eq!(
            result.best_move,
            Some(expected_best_move),
            "ベストムーブが期待したものと異なります。"
        );

        println!("nodes_searched: {:?}", result.nodes_searched);
    }
}
