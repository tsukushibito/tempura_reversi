pub trait GameState: Clone + Eq + std::hash::Hash {
    fn is_terminal(&self) -> bool;
    fn evaluate(&self) -> i32;
    fn generate_children(&self) -> Vec<Self>;
    fn order_evaluate(&self) -> i32;
}
