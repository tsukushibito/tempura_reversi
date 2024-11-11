use reversi_core::{board::Board, Color, Move};

use crate::{GameState, SearchResult};

fn evaluate<B: Board>(state: &GameState<B>, color: Color) -> i32 {
    let black_count = state.board.black_count() as i32;
    let white_count = state.board.white_count() as i32;
    match color {
        Color::Black => black_count - white_count,
        Color::White => white_count - black_count,
    }
}

fn negamax<B: Board>(state: &GameState<B>, depth: usize) -> SearchResult {
    // ノード数をカウント
    let mut nodes_searched = 1;

    // 現在のプレイヤーの有効な手を取得
    let valid_moves = state.board.get_valid_moves(state.player);

    // 終端条件のチェック
    if depth == 0 || valid_moves.is_empty() {
        let score = evaluate(state, state.player);
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

        // 再帰的にnegamaxを呼び出し
        let result = negamax(&new_state, depth - 1);

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
    }

    // 結果を返す
    SearchResult {
        best_move,
        path: best_path,
        nodes_searched,
        score: max_score,
    }
}

#[cfg(test)]
mod tests {
    use reversi_core::{array_board::ArrayBoard, Position};

    use super::*;

    #[test]
    fn test_negamax() {
        // ボードを初期化
        let board = ArrayBoard::new();

        // ゲーム状態を作成
        let state = GameState::new(board, Color::Black);

        // 探索深さを設定
        let depth = 3;

        // negamax関数を呼び出す
        let result = negamax(&state, depth);

        // ベストムーブを表示
        println!("ベストムーブ: {:?}", result.best_move);

        // 期待するベストムーブを定義（例としてC4）
        let expected_best_move = Move {
            position: Some(Position::C4),
            color: Color::Black,
        };

        // アサートで確認
        assert_eq!(
            result.best_move,
            Some(expected_best_move),
            "ベストムーブが期待したものと異なります。"
        );

        // スコアの確認（具体的な値は評価関数とゲーム状態によります）
        // ここではスコアが正の値であることを確認します
        assert!(result.score > 0, "スコアが正の値ではありません。");

        // 探索ノード数が適切か確認
        let max_nodes_searched = 100000;
        assert!(
            result.nodes_searched <= max_nodes_searched,
            "探索ノード数が多すぎます。"
        );
    }
}
