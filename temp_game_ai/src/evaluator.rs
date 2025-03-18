use super::GameState;

pub trait Evaluator<S: GameState> {
    fn evaluate(&mut self, state: &S) -> i32;
}
