pub trait GameState: Default + Clone + Eq + std::hash::Hash {
    type Move: Clone;

    fn valid_moves(&self) -> Vec<Self::Move>;
    fn make_move(&mut self, mv: &Self::Move);
    fn undo_move(&mut self);
}
