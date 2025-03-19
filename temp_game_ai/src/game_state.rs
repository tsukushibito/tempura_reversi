pub trait GameState: Clone + Eq + std::hash::Hash {
    type Move: Clone;

    fn is_terminal(&self) -> bool;
    fn generate_children(&self) -> Vec<(Self, Self::Move)>;
}
