use std::collections::HashMap;
use std::hash::Hash;

use reversi_core::Color;
use reversi_core::{board::Board, Move};

use crate::{GameState, SearchResult};

type EvalFunc<B> = fn(&GameState<B>, Color) -> i32;

pub struct Negaalpha<'a, B: Board + Hash + Eq + Clone> {
    evaluate: EvalFunc<B>,
    transposition_table: HashMap<B, i32>,
    phantom: std::marker::PhantomData<&'a B>,
}

impl<'a, B: Board + Hash + Eq + Clone> Negaalpha<'a, B> {
    fn new(evaluate: EvalFunc<B>) -> Self {
        Negaalpha {
            evaluate,
            transposition_table: HashMap::new(),
            phantom: std::marker::PhantomData,
        }
    }

    fn search(
        &mut self,
        state: &GameState<B>,
        depth: usize,
        mut alpha: i32,
        beta: i32,
    ) -> SearchResult {
        // メモ化テーブルの確認
        if let Some(&score) = self.transposition_table.get(&state.board) {
            return SearchResult {
                best_move: None,
                path: Vec::new(),
                nodes_searched: 0, // 新たなノードは探索していない
                score,
            };
        }

        // ノード数をカウント
        let mut nodes_searched = 1;

        // 現在のプレイヤーの有効な手を取得
        let valid_moves = state.board.get_valid_moves(state.player);

        // 終端条件のチェック
        if depth == 0 || valid_moves.is_empty() {
            let score = (self.evaluate)(state, state.player);
            // スコアをメモ化
            self.transposition_table.insert(state.board.clone(), score);
            return SearchResult {
                best_move: None,
                path: Vec::new(),
                nodes_searched,
                score,
            };
        }

        // ベストスコアとベストムーブの初期化
        let mut max_score = i32::MIN;
        let mut best_move = None;
        let mut best_path = Vec::new();

        // すべての有効な手をループ
        for mv_pos in valid_moves {
            // ボードをクローンして手を適用
            let mut new_board = state.board.clone();
            new_board.make_move(state.player, &mv_pos);

            // 相手のターンで新しいゲーム状態を作成
            let new_state = GameState {
                board: new_board,
                player: state.player.opponent(),
            };

            // 再帰的にsearchを呼び出し
            let result = self.search(&new_state, depth - 1, -beta, -alpha);

            // スコアを反転
            let score = -result.score;

            nodes_searched += result.nodes_searched;

            // ベストスコアの更新
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

            // アルファ値の更新
            if score > alpha {
                alpha = score;
            }

            // ベータカットオフ
            if alpha >= beta {
                break;
            }
        }

        // 結果をメモ化
        self.transposition_table
            .insert(state.board.clone(), max_score);

        // 結果を返す
        SearchResult {
            best_move,
            path: best_path,
            nodes_searched,
            score: max_score,
        }
    }
}

#[cfg(test)]
mod tests {
    use reversi_core::{array_board::ArrayBoard, Position};

    use crate::evaluate::simple_evaluate;

    use super::*;

    #[test]
    fn test_negaalpha() {
        // ボードを初期化
        let board = ArrayBoard::new();

        // ゲーム状態を作成
        let state = GameState::new(board, Color::Black);

        // 評価関数を指定してNegaalphaを作成
        let mut negaalpha = Negaalpha::new(simple_evaluate);

        // 探索深さを設定
        let depth = 5;

        // アルファとベータの初期値を設定
        let alpha = i32::MIN + 1;
        let beta = i32::MAX;

        // 探索を開始
        let result = negaalpha.search(&state, depth, alpha, beta);

        // ベストムーブを表示
        println!("ベストムーブ: {:?}", result.best_move);

        // ベストムーブが期待したものかを確認（例としてD3を期待）
        let expected_best_move = Move {
            position: Some(Position::D3),
            color: Color::Black,
        };

        assert_eq!(
            result.best_move,
            Some(expected_best_move),
            "ベストムーブが期待したものと異なります。"
        );

        // スコアが期待値かを確認（具体的な期待値は評価関数によります）
        assert!(result.score > 0, "スコアが正の値ではありません。");

        // 探索ノード数が適切か確認（アルファベータ法によりノード数が減少しているか）
        let max_nodes_searched = 5000; // ネガマックス法と比較してノード数が減っているか確認
        assert!(
            result.nodes_searched <= max_nodes_searched,
            "探索ノード数が多すぎます。"
        );
    }
}