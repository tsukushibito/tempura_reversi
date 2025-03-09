use std::i32;
use std::{cmp, collections::HashMap};

use super::search_state::SearchState;
use super::Strategy;
use crate::evaluator::{EvaluationFunction, PhaseAwareEvaluator};
use rand::rng;
use rand_distr::{Distribution, Normal};
use temp_reversi_core::{Board, Game, Position};

const CACHE_HIT_BONUS: i32 = 1000;
const INF: i32 = i32::MAX;

/// Strategy implementing Negalpha search with a transposition table and move ordering.
#[derive(Clone)]
pub struct NegaScoutStrategy<E, B>
where
    E: EvaluationFunction<B> + Send + Sync,
    B: Board,
{
    evaluator: E,
    transposition_table_upper: HashMap<SearchState<B>, i32>,
    transposition_table_lower: HashMap<SearchState<B>, i32>,
    former_transposition_table_upper: HashMap<SearchState<B>, i32>,
    former_transposition_table_lower: HashMap<SearchState<B>, i32>,
    visited_nodes: u64,
    max_depth: i32,

    normal: Normal<f64>,
}

impl<E, B> NegaScoutStrategy<E, B>
where
    E: EvaluationFunction<B> + Send + Sync,
    B: Board,
{
    /// Constructor.
    pub fn new(evaluator: E, max_depth: i32, sigma: f64) -> Self {
        Self {
            evaluator,
            transposition_table_upper: HashMap::new(),
            transposition_table_lower: HashMap::new(),
            former_transposition_table_upper: HashMap::new(),
            former_transposition_table_lower: HashMap::new(),
            visited_nodes: 0,
            max_depth,
            normal: Normal::new(0.0, sigma).unwrap(),
        }
    }

    /// Calculates move ordering value using MobilityEvaluator.
    fn calc_move_ordering_value(&self, state: &SearchState<B>) -> i32 {
        if let Some(&score) = self.former_transposition_table_upper.get(state) {
            CACHE_HIT_BONUS - score
        } else if let Some(&score) = self.former_transposition_table_lower.get(state) {
            CACHE_HIT_BONUS - score
        } else {
            let evaluator = PhaseAwareEvaluator::default();
            -evaluator.evaluate(&state.board, state.current_player)
        }
    }

    fn nega_alpha_transpose(
        &mut self,
        state: &SearchState<B>,
        depth: i32,
        passed: bool,
        alpha: i32,
        beta: i32,
    ) -> i32 {
        self.visited_nodes += 1;

        if depth == 0 {
            // Add random fluctuation to the evaluation score
            // (scaled by the total number of stones on the board).
            let stones = state.board.count_stones();
            let total = (stones.0 + stones.1) as f64;
            let mut rng = rng();
            let fluctuation = (self.normal.sample(&mut rng) * total / 30.0) as i32;
            return self.evaluator.evaluate(&state.board, state.current_player) + fluctuation;
        }

        // let mut upper = INF;
        // if let Some(&score) = self.transposition_table_upper.get(&state) {
        //     upper = score;
        // }
        let upper = self
            .transposition_table_upper
            .get(&state)
            .copied()
            .unwrap_or(INF);
        let lower = self
            .transposition_table_lower
            .get(&state)
            .copied()
            .unwrap_or(-INF);

        if upper == lower {
            return upper;
        }

        let alpha = cmp::max(alpha, lower);
        let beta = cmp::min(beta, upper);

        let valid_moves = state.board.valid_moves(state.current_player);
        if valid_moves.is_empty() {
            if passed {
                // Game over
                return self.evaluator.evaluate(&state.board, state.current_player);
            }

            // Pass
            let next_state =
                SearchState::<B>::new(state.board.clone(), state.current_player.opponent());
            return -self.nega_alpha_transpose(&next_state, depth, true, -beta, -alpha);
        }

        // Generate children and sort them by move ordering value
        let mut children: Vec<(Position, SearchState<B>, i32)> = Vec::new();
        for pos in valid_moves {
            if let Some(child_state) = state.apply_move(pos) {
                let ordering = self.calc_move_ordering_value(&child_state);
                children.push((pos, child_state, ordering));
            }
        }
        if children.len() >= 2 {
            children.sort_by(|a, b| b.2.cmp(&a.2));
        }

        let mut max_score = -INF;
        let mut alpha_local = alpha;
        for (_, child_state, _) in children.iter() {
            let score =
                -self.nega_alpha_transpose(child_state, depth - 1, false, -beta, -alpha_local);
            if score >= beta {
                if score > lower {
                    self.transposition_table_lower.insert(state.clone(), score);
                }
                return score;
            }

            alpha_local = cmp::max(alpha, score);
            max_score = cmp::max(max_score, score);
        }

        if max_score < alpha_local {
            self.transposition_table_upper
                .insert(state.clone(), max_score);
        } else {
            self.transposition_table_upper
                .insert(state.clone(), max_score);
            self.transposition_table_lower
                .insert(state.clone(), max_score);
        }

        max_score
    }

    fn nega_scout(
        &mut self,
        state: &SearchState<B>,
        depth: i32,
        passed: bool,
        alpha: i32,
        beta: i32,
    ) -> i32 {
        self.visited_nodes += 1;

        if depth == 0 {
            // Add random fluctuation to the evaluation score
            // (scaled by the total number of stones on the board).
            let stones = state.board.count_stones();
            let total = (stones.0 + stones.1) as f64;
            let mut rng = rng();
            let fluctuation = (self.normal.sample(&mut rng) * total / 64.0) as i32;
            return self.evaluator.evaluate(&state.board, state.current_player) + fluctuation;
        }

        let upper = self
            .transposition_table_upper
            .get(&state)
            .copied()
            .unwrap_or(INF);
        let lower = self
            .transposition_table_lower
            .get(&state)
            .copied()
            .unwrap_or(-INF);

        if upper == lower {
            return upper;
        }

        let alpha = cmp::max(alpha, lower);
        let beta = cmp::min(beta, upper);

        let valid_moves = state.board.valid_moves(state.current_player);
        if valid_moves.is_empty() {
            if passed {
                // Game over
                return self.evaluator.evaluate(&state.board, state.current_player);
            }

            // Pass
            let next_state =
                SearchState::<B>::new(state.board.clone(), state.current_player.opponent());
            return -self.nega_scout(&next_state, depth, true, -beta, -alpha);
        }

        // Generate children and sort them by move ordering value
        let mut children: Vec<(Position, SearchState<B>, i32)> = Vec::new();
        for pos in valid_moves {
            if let Some(child_state) = state.apply_move(pos) {
                let ordering = self.calc_move_ordering_value(&child_state);
                children.push((pos, child_state, ordering));
            }
        }
        if children.len() >= 2 {
            children.sort_by(|a, b| b.2.cmp(&a.2));
        }

        // 最善手候補は通常窓で探索
        let first_child = children.first().expect("No children found.");
        let score = -self.nega_scout(&first_child.1, depth - 1, false, -beta, -alpha);
        if score >= beta {
            if score > lower {
                self.transposition_table_lower.insert(state.clone(), score);
            }
            return score;
        }

        let mut max_score = score;
        let mut alpha_local = cmp::max(alpha, score);

        // 次善手以降は窓を狭めて探索
        for (_, child_state, _) in children.iter().skip(1) {
            let mut score = -self.nega_alpha_transpose(
                child_state,
                depth - 1,
                false,
                -alpha_local - 1,
                -alpha_local,
            );

            // Fail-High
            if score >= beta {
                if score > lower {
                    self.transposition_table_lower.insert(state.clone(), score);
                }
                return score;
            }

            if score > alpha_local {
                // より良い手が見つかった場合、再探索
                alpha_local = score;
                score = -self.nega_scout(child_state, depth - 1, false, -beta, -alpha_local);
                // Fail-High
                if score >= beta {
                    if score > lower {
                        self.transposition_table_lower.insert(state.clone(), score);
                    }
                    return score;
                }
            }

            alpha_local = cmp::max(alpha, score);
            max_score = cmp::max(max_score, score);
        }

        if max_score < alpha_local {
            self.transposition_table_upper
                .insert(state.clone(), max_score);
        } else {
            self.transposition_table_upper
                .insert(state.clone(), max_score);
            self.transposition_table_lower
                .insert(state.clone(), max_score);
        }

        max_score
    }

    /// Finds the best move using iterative deepening search.
    fn search(&mut self, state: SearchState<B>) -> Option<Position> {
        self.visited_nodes = 0;
        self.transposition_table_upper.clear();
        self.transposition_table_lower.clear();
        self.former_transposition_table_upper.clear();
        self.former_transposition_table_lower.clear();

        let valid_moves = state.board.valid_moves(state.current_player);
        if valid_moves.is_empty() {
            return None;
        }

        let mut best_move = None;
        let start_depth = cmp::max(1, self.max_depth - 3);
        for depth in start_depth..=self.max_depth {
            let mut alpha = -INF;
            let beta = INF;
            let mut children: Vec<(Position, SearchState<B>, i32)> = Vec::new();

            for pos in &valid_moves {
                if let Some(child_state) = state.apply_move(*pos) {
                    let ordering = self.calc_move_ordering_value(&child_state);
                    children.push((*pos, child_state, ordering));
                }
            }

            if children.len() >= 2 {
                children.sort_by(|a, b| b.2.cmp(&a.2));
            }

            // 最善手候補は通常窓で探索
            let score = -self.nega_scout(&children[0].1, depth - 1, false, -beta, -alpha);
            alpha = score;
            best_move = Some(children[0].0);

            // 次善手以降は窓を狭めて探索
            for (pos, child_state, _) in children.iter().skip(1) {
                let score =
                    -self.nega_alpha_transpose(child_state, depth - 1, false, -alpha - 1, -alpha);

                if alpha < score {
                    // 良い手が見つかった場合、再探索
                    alpha = score;
                    let _score = -self.nega_scout(child_state, depth - 1, false, -beta, -alpha);
                    best_move = Some(*pos);
                }

                alpha = cmp::max(alpha, score);
            }

            // Use the transposition table for move ordering
            std::mem::swap(
                &mut self.transposition_table_upper,
                &mut self.former_transposition_table_upper,
            );
            self.transposition_table_upper.clear();
            std::mem::swap(
                &mut self.transposition_table_lower,
                &mut self.former_transposition_table_lower,
            );
            self.transposition_table_lower.clear();

            // Print for debug
            // println!(
            //     "Depth {}: best_move: {:?}, visited_nodes: {}",
            //     depth, best_move, self.visited_nodes
            // );
        }
        best_move
    }
}

impl<E, B> Strategy<B> for NegaScoutStrategy<E, B>
where
    E: EvaluationFunction<B> + Send + Sync + Clone + 'static,
    B: Board + Send + Sync + 'static,
{
    /// Evaluates the game state and decides the next move.
    fn evaluate_and_decide(&mut self, game: &Game<B>) -> Option<Position> {
        let state = SearchState::<B>::new(game.board_state().clone(), game.current_player());
        self.search(state)
    }

    fn clone_box(&self) -> Box<dyn Strategy<B>> {
        Box::new((*self).clone())
    }
}

#[cfg(test)]
mod tests {
    use temp_reversi_core::{Bitboard, Player};

    use crate::{evaluator::TempuraEvaluator, strategy::NegaAlphaTTStrategy};

    use super::*;

    #[test]
    fn test_visited_nodes() {
        let depth = 10;

        // let game = Game::default();
        // let evaluator = PhaseAwareEvaluator::default();
        // let mut strategy = NegaAlphaStrategy::new(evaluator, depth);

        // let start = std::time::Instant::now();
        // strategy.evaluate_and_decide(&game);
        // let elapsed = start.elapsed();
        // println!("[NegaAlpha] Elapsed: {:?}", elapsed);
        // assert!(
        //     strategy.nodes_searched > 0,
        //     "Nodes searched should be greater than 0."
        // );
        // println!("[NegaAlpha] Visited nodes: {}", strategy.nodes_searched);

        let mut game = Game::<Bitboard>::default();
        let valid_moves = game.valid_moves();
        game.apply_move(valid_moves[0]).unwrap();
        let valid_moves = game.valid_moves();
        game.apply_move(valid_moves[0]).unwrap();
        let evaluator = TempuraEvaluator::new("../gen0/models/temp_model.bin");
        let mut strategy = NegaAlphaTTStrategy::new(evaluator, depth, 0.0);

        let start = std::time::Instant::now();
        strategy.evaluate_and_decide(&game);
        let elapsed = start.elapsed();
        println!("[NegaAlphaTT] Elapsed: {:?}", elapsed);
        assert!(
            strategy.visited_nodes > 0,
            "Visited nodes should be greater than 0."
        );
        println!("[NegaAlphaTT] Visited nodes: {}", strategy.visited_nodes);

        let mut game = Game::<Bitboard>::default();
        let valid_moves = game.valid_moves();
        game.apply_move(valid_moves[0]).unwrap();
        let valid_moves = game.valid_moves();
        game.apply_move(valid_moves[0]).unwrap();
        let evaluator = TempuraEvaluator::new("../gen0/models/temp_model.bin");
        let mut strategy = NegaScoutStrategy::new(evaluator, depth, 0.0);

        let start = std::time::Instant::now();
        strategy.evaluate_and_decide(&game);
        let elapsed = start.elapsed();
        println!("[NegaScout] Elapsed: {:?}", elapsed);
        assert!(
            strategy.visited_nodes > 0,
            "Visited nodes should be greater than 0."
        );
        println!("[NegaScout] Visited nodes: {}", strategy.visited_nodes);
    }

    #[test]
    fn test_self_play() {
        let mut game = Game::<Bitboard>::default();
        let evaluator = TempuraEvaluator::new("../gen0/models/temp_model.bin");
        let mut strategy = NegaScoutStrategy::new(evaluator, 6, 0.0);

        let start = std::time::Instant::now();
        while !game.is_game_over() {
            if let Some(chosen_move) = strategy.evaluate_and_decide(&game) {
                game.apply_move(chosen_move).unwrap();
            } else {
                break;
            }
        }
        let elapsed = start.elapsed();

        println!("[NegaScout] Elapsed: {:?}", elapsed);

        let mut game = Game::<Bitboard>::default();
        let evaluator = TempuraEvaluator::new("../gen0/models/temp_model.bin");
        let mut strategy = NegaAlphaTTStrategy::new(evaluator, 6, 0.0);

        let start = std::time::Instant::now();
        while !game.is_game_over() {
            if let Some(chosen_move) = strategy.evaluate_and_decide(&game) {
                game.apply_move(chosen_move).unwrap();
            } else {
                break;
            }
        }
        let elapsed = start.elapsed();

        println!("[NegaAlphaTT] Elapsed: {:?}", elapsed);
    }

    // DummyBoard: Board トレイトの簡易実装
    #[derive(Clone, Default, PartialEq, Eq)]
    pub struct DummyBoard {
        pub value: i32,
        pub children: Vec<DummyBoard>,
    }
    impl DummyBoard {
        pub fn new(value: i32, children: Vec<DummyBoard>) -> Self {
            Self { value, children }
        }
    }
    impl std::fmt::Display for DummyBoard {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DummyBoard(value: {})", self.value)
        }
    }
    impl Board for DummyBoard {
        fn bits(&self) -> (u64, u64) {
            (0, 0)
        }
        fn valid_moves(&self, _player: Player) -> Vec<Position> {
            // 有効手として、children のインデックスを返す
            (0..self.children.len() as u8)
                .map(|n| Position::from_u8(n).unwrap())
                .collect()
        }
        fn count_stones(&self) -> (usize, usize) {
            (0, 0)
        }
        fn is_game_over(&self) -> bool {
            self.children.is_empty()
        }
        fn apply_move(&mut self, position: Position, _player: Player) -> Result<(), &'static str> {
            // 指定された位置が有効か確認
            if let Some(child) = self.children.get(position.to_u8() as usize) {
                // 子状態と同じ状態に更新
                *self = child.clone();
                Ok(())
            } else {
                panic!("無効なムーブ位置: {}", position);
            }
        }
        fn get_hash(&self) -> u64 {
            self.value as u64
        }
    }
    trait DummyValue {
        fn value(&self) -> i32;
    }
    impl DummyValue for DummyBoard {
        fn value(&self) -> i32 {
            self.value
        }
    }

    // DummyEvaluator: 評価関数は board.value を返す
    #[derive(Clone)]
    pub struct DummyEvaluator;
    impl<B: Board + DummyValue> EvaluationFunction<B> for DummyEvaluator {
        fn evaluate(&self, board: &B, _current_player: Player) -> i32 {
            board.value()
        }
    }

    // テスト対象の型エイリアス
    type TestNegaScout = NegaScoutStrategy<DummyEvaluator, DummyBoard>;
    const INF: i32 = std::i32::MAX;

    // １．深さ0のテスト
    #[test]
    fn test_depth_zero() {
        let evaluator = DummyEvaluator;
        // sigma=0.0 により乱数による変動は除去
        let mut strategy = TestNegaScout::new(evaluator, 1, 0.0);
        // 子局面がない葉局面、評価値 42
        let board = DummyBoard::new(42, vec![]);
        let state = SearchState::new(board, Player::Black);
        let result = strategy.nega_scout(&state, 0, false, -INF, INF);
        let expected = strategy
            .evaluator
            .evaluate(&state.board, state.current_player);
        assert_eq!(result, expected, "深さ0のテストで評価値が一致しません");
    }

    // ２．ターミナル状態（全パス）のテスト
    #[test]
    fn test_terminal_state() {
        let evaluator = DummyEvaluator;
        let mut strategy = TestNegaScout::new(evaluator, 1, 0.0);
        // 子局面がないので、ムーブ候補が空。passed フラグが true の場合は評価値を返す
        let board = DummyBoard::new(55, vec![]);
        let state = SearchState::new(board, Player::Black);
        let result = strategy.nega_scout(&state, 1, true, -INF, INF);
        let expected = strategy
            .evaluator
            .evaluate(&state.board, state.current_player);
        assert_eq!(
            result, expected,
            "ターミナル状態のテストで評価値が一致しません"
        );
    }

    // ３．既知局面での最小値／最大値のテスト
    #[test]
    fn test_known_minimax() {
        let evaluator = DummyEvaluator;
        // ルート局面の子局面として、評価値 100 と 50 の 2局面を構築
        let child0 = DummyBoard::new(100, vec![]);
        let child1 = DummyBoard::new(50, vec![]);
        let root = DummyBoard::new(0, vec![child0, child1]);
        let state = SearchState::new(root, Player::Black);
        // negamax により、各子は -評価値となるので、子0: -100, 子1: -50 となり、最大値は -50 となるはず
        let mut strategy = TestNegaScout::new(evaluator, 1, 0.0);
        let result = strategy.nega_scout(&state, 1, false, -INF, INF);
        let expected = -50;
        assert_eq!(
            result, expected,
            "既知局面での最小値／最大値のテストで値が一致しません"
        );
    }

    // ４．βカットオフのテスト
    #[test]
    fn test_beta_cutoff() {
        let evaluator = DummyEvaluator;
        // ルート局面に1手の子局面のみを持つ局面
        // 子局面（葉）の評価値が -100 なら、negamax では 100 となるので、
        // 例えば beta = 50 とすれば 100 >= 50 となりカットオフが発生するはず
        let child = DummyBoard::new(-100, vec![]);
        let root = DummyBoard::new(0, vec![child]);
        let state = SearchState::new(root, Player::Black);
        let mut strategy = TestNegaScout::new(evaluator, 1, 0.0);
        let result = strategy.nega_scout(&state, 1, false, -INF, 50);
        assert_eq!(result, 100, "βカットオフのテストで返値が想定と異なります");
    }

    // ５．再現性と置換表キャッシュのテスト
    #[test]
    fn test_reproducibility_and_caching() {
        let evaluator = DummyEvaluator;
        let mut strategy = TestNegaScout::new(evaluator, 2, 0.0);
        // ルート局面に1手の子局面を持つ単純な局面
        let child = DummyBoard::new(75, vec![]);
        let root = DummyBoard::new(0, vec![child]);
        let state = SearchState::new(root, Player::Black);
        // 1回目の探索結果と訪問ノード数を記録
        let result1 = strategy.nega_scout(&state, 1, false, -INF, INF);
        let visited1 = strategy.visited_nodes;
        // 2回目の探索（同一局面）を実行
        strategy.visited_nodes = 0;
        let result2 = strategy.nega_scout(&state, 1, false, -INF, INF);
        let visited2 = strategy.visited_nodes;
        assert_eq!(result1, result2, "再現性のテストで結果が一致しません");
        // キャッシュ利用により、2回目は訪問ノード数が減少していることを期待
        assert!(
            visited2 <= visited1,
            "置換表キャッシュのテストで、訪問ノード数が減少していません"
        );
    }
}
