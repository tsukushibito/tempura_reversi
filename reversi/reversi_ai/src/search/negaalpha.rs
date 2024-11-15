use std::collections::HashMap;
use std::hash::Hash;

use reversi_core::board::BOARD_SIZE;
use reversi_core::Color;
use reversi_core::{board::Board, Move, Position};

use crate::{GameState, SearchResult};

type EvalFunc<B> = fn(&GameState<B>, Color) -> i32;

pub struct TranspositionTableEntry {
    pub score: i32,
    pub depth: u8,
    pub best_move: i8,
    pub policy: [i32; BOARD_SIZE * BOARD_SIZE],
}

pub struct Negaalpha<B: Board + Hash + Eq + Clone> {
    evaluate: EvalFunc<B>,
    transposition_table: HashMap<B, TranspositionTableEntry>,
    use_move_ordering: bool,
}

impl<B: Board + Hash + Eq + Clone> Negaalpha<B> {
    pub fn new(evaluate: EvalFunc<B>) -> Self {
        Negaalpha {
            evaluate,
            transposition_table: HashMap::new(),
            use_move_ordering: true,
        }
    }

    pub fn set_move_ordering(&mut self, enabled: bool) {
        self.use_move_ordering = enabled;
    }

    fn evaluate_move(&self, state: &GameState<B>, pos: &Position) -> i32 {
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
        state: &GameState<B>,
        depth: u8,
        mut alpha: i32,
        beta: i32,
    ) -> SearchResult {
        if let Some(entry) = self.transposition_table.get(&state.board) {
            if entry.depth >= depth {
                return SearchResult {
                    best_move: Some(Move {
                        position: Some(Position::from_index(entry.best_move)),
                        color: state.player,
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

        let mut valid_moves = state.board.get_valid_moves(state.player);

        if depth == 0 || valid_moves.is_empty() {
            let score = (self.evaluate)(state, state.player);
            self.transposition_table.insert(
                state.board.clone(),
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
            valid_moves.sort_by_cached_key(|pos| -self.evaluate_move(state, pos));
        }

        let mut max_score = i32::MIN;
        let mut best_move = None;
        let mut best_path = Vec::new();

        for mv_pos in valid_moves {
            let mut new_board = state.board.clone();
            new_board.make_move(state.player, &mv_pos);

            let new_state = GameState {
                board: new_board,
                player: state.player.opponent(),
            };

            let result = self.search(&new_state, depth - 1, -beta, -alpha);

            let score = -result.score;

            nodes_searched += result.nodes_searched;

            let index = mv_pos.to_index();
            policy[index as usize] = score;

            if score > max_score {
                max_score = score;
                best_move = Some(Move {
                    position: Some(mv_pos),
                    color: state.player,
                });
                best_path = vec![Move {
                    position: Some(mv_pos),
                    color: state.player,
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
            if let Some(p) = bm.position {
                p.to_index()
            } else {
                -1
            }
        } else {
            -1
        };
        self.transposition_table.insert(
            state.board.clone(),
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
    use reversi_core::{array_board::ArrayBoard, Position};

    use crate::evaluate::simple_evaluate;

    use super::*;

    #[test]
    fn test_negaalpha_no_move_ordering() {
        let board = ArrayBoard::new();
        let state = GameState::new(board, Color::Black);

        let mut negaalpha = Negaalpha::new(simple_evaluate);
        negaalpha.set_move_ordering(false);

        let depth = 9;

        let alpha = i32::MIN + 1;
        let beta = i32::MAX;

        let result = negaalpha.search(&state, depth, alpha, beta);

        println!("best_move: {:?}", result.best_move);

        println!("path: ");
        let mut board = state.board.clone();
        board.display();
        for mov in result.path {
            board.make_move(mov.color, &mov.position.unwrap());
            board.display();
        }
        println!();

        let expected_best_move = Move {
            position: Some(Position::D3),
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
        let board = ArrayBoard::new();
        let state = GameState::new(board, Color::Black);

        let mut negaalpha = Negaalpha::new(simple_evaluate);
        negaalpha.set_move_ordering(true);

        let depth = 9;

        let alpha = i32::MIN + 1;
        let beta = i32::MAX;

        let result = negaalpha.search(&state, depth, alpha, beta);

        println!("best_move: {:?}", result.best_move);

        println!("path: ");
        let mut board = state.board.clone();
        board.display();
        for mov in result.path {
            board.make_move(mov.color, &mov.position.unwrap());
            board.display();
        }
        println!();

        let expected_best_move = Move {
            position: Some(Position::D3),
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
