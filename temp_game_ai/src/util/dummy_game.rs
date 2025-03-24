use crate::{Evaluator, GameState};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DummyMove {
    A,
    B,
    C,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct DummyGame {
    pub history: Vec<DummyMove>,
}

impl Hash for DummyGame {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for mv in &self.history {
            mv.hash(state);
        }
    }
}

const MAX_DEPTH: usize = 3;

impl DummyGame {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    pub fn compute_score(&self) -> i32 {
        let mut score = 0;
        for mv in &self.history {
            let value = match mv {
                DummyMove::A => 0,
                DummyMove::B => 1,
                DummyMove::C => 2,
            };
            score = score * 3 + value;
        }
        let remaining = MAX_DEPTH.saturating_sub(self.history.len());
        score *= 3_usize.pow(remaining as u32);
        score as i32 + 1
    }
}

impl GameState for DummyGame {
    type Move = DummyMove;

    fn valid_moves(&self) -> Vec<Self::Move> {
        // どの状態でも常に3手 (A, B, C) が選択可能とする
        // vec![DummyMove::A, DummyMove::B, DummyMove::C]
        vec![DummyMove::B, DummyMove::A, DummyMove::C]
    }

    fn make_move(&mut self, mv: &Self::Move) {
        self.history.push(mv.clone());
    }

    fn undo_move(&mut self) {
        self.history.pop();
    }
}

pub struct DummyEvaluator;

impl Evaluator<DummyGame> for DummyEvaluator {
    fn evaluate(&mut self, state: &DummyGame) -> i32 {
        // 探索の深さが葉（例えば深さ3）に達したときに、履歴からユニークなスコアを返す
        state.compute_score()
    }
}

pub struct OptimalOrderingEvaluator;

impl OptimalOrderingEvaluator {
    /// 状態 `state` から、残りの深さ（MAX_DEPTH - 現在の手数）分の完全探索によるネガ・マックス値を返す。
    fn perfect_negamax(state: &mut DummyGame, depth: usize) -> i32 {
        if depth == 0 {
            return state.compute_score();
        }
        let mut best = i32::MIN;
        for mv in state.valid_moves() {
            state.make_move(&mv);
            let score = -Self::perfect_negamax(state, depth - 1);
            state.undo_move();
            best = best.max(score);
        }
        best
    }
}

impl Evaluator<DummyGame> for OptimalOrderingEvaluator {
    fn evaluate(&mut self, state: &DummyGame) -> i32 {
        // 状態は変更しないので、cloneして探索を行う
        let mut cloned = state.clone();
        // 残りの深さ＝MAX_DEPTH - 現在の手数
        let remaining_depth = MAX_DEPTH.saturating_sub(cloned.history.len());
        Self::perfect_negamax(&mut cloned, remaining_depth)
    }
}
