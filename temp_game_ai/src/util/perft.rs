use crate::GameState;

pub fn perft<S>(state: &S, depth: usize, passed: bool) -> usize
where
    S: GameState,
{
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;
    let children = state.generate_children();

    if !children.is_empty() {
        if depth == 1 {
            return children.len();
        }
        for child in &children {
            nodes += perft(&child.0, depth - 1, false);
        }
    } else {
        if passed {
            return 1;
        } else {
            let next_state = state.switch_player();
            nodes += perft(&next_state, depth - 1, true);
        }
    }

    nodes
}
