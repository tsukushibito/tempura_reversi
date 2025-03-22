use crate::GameState;

pub fn perft<S>(state: &S, depth: usize) -> usize
where
    S: GameState,
{
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;
    let children = state.generate_children();

    for child in &children {
        nodes += perft(&child.0, depth - 1);
    }

    nodes
}
