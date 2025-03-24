#![cfg(test)]

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
        score + 1 // 最小値が1となるようにする
    }
}

impl GameState for DummyGame {
    type Move = DummyMove;

    fn valid_moves(&self) -> Vec<Self::Move> {
        // どの状態でも常に3手 (A, B, C) が選択可能とする
        vec![DummyMove::A, DummyMove::B, DummyMove::C]
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
