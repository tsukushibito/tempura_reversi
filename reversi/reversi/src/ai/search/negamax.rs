use std::collections::HashMap;

use crate::{
    ai::SearchResult,
    bit_board::BitBoard,
    board::{Board, BOARD_SIZE},
    Color, Move,
};

type EvalFunc = fn(&BitBoard, Color) -> i32;

pub struct Negamax {
    evaluate: EvalFunc,
    transposition_table: HashMap<BitBoard, i32>,
}

impl Negamax {
    pub fn new(evaluate: EvalFunc) -> Self {
        Negamax {
            evaluate,
            transposition_table: HashMap::new(),
        }
    }

    pub fn search(&mut self, board: &BitBoard, player: Color, depth: u8) -> SearchResult {
        // メモ化テーブルの確認
        if let Some(&score) = self.transposition_table.get(&board) {
            return SearchResult {
                best_move: None,
                path: Vec::new(),
                nodes_searched: 0, // 新たなノードは探索していない
                score,
                policy: [0; BOARD_SIZE * BOARD_SIZE],
            };
        }

        // ノード数をカウント
        let mut nodes_searched = 1;

        // 現在のプレイヤーの有効な手を取得
        let valid_moves = board.get_valid_moves(player);

        // 終端条件のチェック
        if depth == 0 || valid_moves.is_empty() {
            let score = (self.evaluate)(&board, player);
            // スコアをメモ化
            self.transposition_table.insert(board.clone(), score);
            return SearchResult {
                best_move: None,
                path: Vec::new(),
                nodes_searched,
                score,
                policy: [0; BOARD_SIZE * BOARD_SIZE],
            };
        }

        // ベストスコアとベストムーブの初期化
        let mut max_score = i32::MIN;
        let mut best_move = None;
        let mut best_path = Vec::new();

        // すべての有効な手をループ
        for mv_pos in valid_moves {
            // ボードをクローンして手を適用
            let mut new_board = board.clone();
            new_board.make_move(player, &mv_pos);

            // 再帰的にsearchを呼び出し
            let result = self.search(&new_board, player.opponent(), depth - 1);

            // スコアを反転
            let score = -result.score;

            nodes_searched += result.nodes_searched;

            // ベストスコアの更新
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
        }

        // 結果をメモ化
        self.transposition_table.insert(board.clone(), max_score);

        // 結果を返す
        SearchResult {
            best_move,
            path: best_path,
            nodes_searched,
            score: max_score,
            policy: [0; BOARD_SIZE * BOARD_SIZE],
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{Color, Position};

    use crate::ai::evaluate::simple_evaluate;

    use super::*;

    #[test]
    fn test_negamax() {
        let board = BitBoard::new();

        let depth = 7;

        let mut negamax = Negamax::new(simple_evaluate);
        let result = negamax.search(&board, Color::Black, depth);

        println!("best_move: {:?}", result.best_move);

        println!("path: ");
        let mut board = board.clone();
        board.display();
        for mov in result.path {
            board.make_move(mov.color, &mov.position);
            board.display();
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

        println!("nodes_searched: {:?}", result.nodes_searched)
    }
}
