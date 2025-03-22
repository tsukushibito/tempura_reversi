pub trait GameState: Default + Clone + Eq + std::hash::Hash {
    type Move: Clone;

    fn generate_children(&self) -> Vec<(Self, Self::Move)>;
}
