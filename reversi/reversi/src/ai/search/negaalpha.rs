use std::collections::HashMap;

use crate::ai::SearchResult;
use crate::bit_board::BitBoard;
use crate::board::{Board, BOARD_SIZE};
use crate::{Color, Move, Position};

type EvalFunc = fn(&BitBoard, Color) -> i32;

pub struct TranspositionTableEntry {
    pub score: i32,
    pub depth: u8,
    pub best_move: i8,
    pub policy: [i32; BOARD_SIZE * BOARD_SIZE],
}

pub struct Negaalpha {
    evaluate: EvalFunc,
    transposition_table: HashMap<BitBoard, TranspositionTableEntry>,
    use_move_ordering: bool,
}

impl Negaalpha {
    pub fn new(evaluate: EvalFunc) -> Self {
        Negaalpha {
            evaluate,
            transposition_table: HashMap::new(),
            use_move_ordering: true,
        }
    }

    pub fn set_move_ordering(&mut self, enabled: bool) {
        self.use_move_ordering = enabled;
    }

    fn evaluate_move(&self, _board: &BitBoard, pos: &Position) -> i32 {
        const POSITION_WEIGHTS: [[i32; 8]; 8] = [
            [100, -20, 10, 5, 5, 10, -20, 100],
            [-20, -50, -2, -2, -2, -2, -50, -20],
            [10, -2, -1, -1, -1, -1, -2, 10],
            [5, -2, -1, -1, -1, -1, -2, 5],
            [5, -2, -1, -1, -1, -1, -2, 5],
            [10, -2, -1, -1, -1, -1, -2, 10],
            [-20, -50, -2, -2, -2, -2, -50, -20],
            [100, -20, 10, 5, 5, 10, -20, 100],
        ];

        let x = pos.x as usize;
        let y = pos.y as usize;
        POSITION_WEIGHTS[y][x]
    }

    // fn evaluate_move(&self, state: &GameState<B>, pos: &Position) -> i32 {
    //     let mut board = state.board.clone();
    //     board.make_move(state.player, pos);
    //     let my_moves = board.get_valid_moves(state.player).len() as i32;
    //     let opponent_moves = board.get_valid_moves(state.player.opponent()).len() as i32;
    //     my_moves - opponent_moves
    // }

    pub fn search(
        &mut self,
        board: &BitBoard,
        player: Color,
        depth: u8,
        mut alpha: i32,
        beta: i32,
    ) -> SearchResult {
        if let Some(entry) = self.transposition_table.get(board) {
            if entry.depth >= depth {
                return SearchResult {
                    best_move: Some(Move {
                        position: Position::from_index(entry.best_move as usize),
                        color: player,
                    }),
                    path: Vec::new(),
                    nodes_searched: 0,
                    score: entry.score,
                    policy: entry.policy,
                };
            }
        }

        let mut nodes_searched = 1;
        let mut policy = [0; BOARD_SIZE * BOARD_SIZE];

        let mut valid_moves = board.get_valid_moves(player);

        if depth == 0 || valid_moves.is_empty() {
            let score = (self.evaluate)(board, player);
            self.transposition_table.insert(
                board.clone(),
                TranspositionTableEntry {
                    score,
                    depth,
                    best_move: -1,
                    policy: [0; 64],
                },
            );
            return SearchResult {
                best_move: None,
                path: Vec::new(),
                nodes_searched,
                score,
                policy,
            };
        }

        if self.use_move_ordering {
            valid_moves.sort_by_cached_key(|pos| -self.evaluate_move(board, pos));
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
            policy[index as usize] = score;

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
        self.transposition_table.insert(
            board.clone(),
            TranspositionTableEntry {
                score: max_score,
                depth,
                best_move: best_move_index,
                policy,
            },
        );

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
    use crate::{bit_board::BitBoard, Position};

    use crate::ai::evaluate::simple_evaluate;

    use super::*;

    #[test]
    fn test_negaalpha_no_move_ordering() {
        let bit_board = BitBoard::init_board();

        let mut negaalpha = Negaalpha::new(simple_evaluate);
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

        let mut negaalpha = Negaalpha::new(simple_evaluate);
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
