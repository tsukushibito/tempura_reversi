#![cfg(test)]
use crate::GameState;

pub fn perft<S>(state: &mut S, depth: usize) -> usize
where
    S: GameState,
{
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;
    let moves = state.valid_moves();
    for mv in moves {
        state.make_move(&mv);
        nodes += perft(state, depth - 1);
        state.undo_move();
    }

    nodes
}
