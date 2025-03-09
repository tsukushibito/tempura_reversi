use super::GameState;

pub trait Evaluator<S: GameState> {
    fn evaluate(&mut self, state: &S) -> i32;
    fn order_evaluate(&self, state: &S) -> i32;
}
