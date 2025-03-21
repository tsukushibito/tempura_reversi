pub trait GameState: Default + Clone + Eq + std::hash::Hash {
    type Move: Clone;

    fn is_terminal(&self) -> bool;
    fn generate_children(&self) -> Vec<(Self, Self::Move)>;
    fn is_pass(&self) -> bool {
        self.generate_children().is_empty()
    }
    fn switch_player(&self) -> Self;
}
