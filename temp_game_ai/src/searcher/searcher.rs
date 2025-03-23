use crate::GameState;

pub trait Searcher<S>
where
    S: GameState,
{
    fn search(&mut self, state: &mut S, max_depth: usize) -> Option<(S::Move, i32)>;
}
