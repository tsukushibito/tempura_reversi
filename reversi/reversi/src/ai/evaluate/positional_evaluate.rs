use crate::{
    board::{Board, BOARD_SIZE},
    game_play::GameState,
    Color, Position,
};

pub fn positional_evaluate<B: Board>(state: &GameState<B>, color: Color) -> i32 {
    let weights: [[i32; BOARD_SIZE]; BOARD_SIZE] = [
        [100, -20, 10, 5, 5, 10, -20, 100],
        [-20, -50, -2, -2, -2, -2, -50, -20],
        [10, -2, -1, -1, -1, -1, -2, 10],
        [5, -2, -1, -1, -1, -1, -2, 5],
        [5, -2, -1, -1, -1, -1, -2, 5],
        [10, -2, -1, -1, -1, -1, -2, 10],
        [-20, -50, -2, -2, -2, -2, -50, -20],
        [100, -20, 10, 5, 5, 10, -20, 100],
    ];

    let mut score = 0;

    (0..BOARD_SIZE).for_each(|y| {
        (0..BOARD_SIZE).for_each(|x| {
            let pos = Position {
                x: x as i8,
                y: y as i8,
            };
            if let Some(c) = state.board.get_disc(&pos) {
                if c == color {
                    score += weights[y][x];
                } else {
                    score -= weights[y][x];
                }
            }
        });
    });
    score
}
