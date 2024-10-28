use reversi_core::{board::Board, Move};

use crate::GameState;

fn negamax<F, B>(
    game_state: &GameState<B>,
    depth: i32,
    eval_func: &F,
    nodes_searched: &mut usize,
    path: &mut Vec<Move>,
) -> i32
where
    F: Fn(&GameState<B>) -> i32,
    B: Board,
{
    *nodes_searched += 1;

    if depth == 0 {
        return eval_func(game_state);
    }

    let mut max_eval = i32::MIN;
    let mut best_move = None;

    let valid_moves = game_state.board.get_valid_moves(game_state.player);
    if valid_moves.is_empty() {
        let new_game_state = GameState {
            board: game_state.board.clone(),
            player: game_state.player.opposite(),
        };
        return -negamax(&new_game_state, depth - 1, eval_func, nodes_searched, path);
    } else {
        for pos in game_state.board.get_valid_moves(game_state.player) {
            let mut new_board = game_state.board.clone();
            new_board.make_move(game_state.player, &pos);
            let new_game_state = GameState {
                board: new_board,
                player: game_state.player.opposite(),
            };
            let m = Move {
                position: Some(pos),
                color: game_state.player,
            };
            path.push(m);

            let score = -negamax(&new_game_state, depth - 1, eval_func, nodes_searched, path);

            path.pop();

            if score > max_eval {
                max_eval = score;
                best_move = Some(m);
            }
        }

        if let Some(m) = best_move {
            path.push(m);
        }

        max_eval
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reversi_core::{array_board::ArrayBoard, Color};

    #[test]
    fn test_negamax() {
        let board = ArrayBoard::new();
        let game_state = GameState::new(board, Color::Black);

        let eval_func = |game_state: &GameState<ArrayBoard>| -> i32 {
            game_state.board.black_count() as i32 - game_state.board.white_count() as i32
        };

        let mut nodes_searched = 0;
        let mut path = Vec::new();
        let score = negamax(&game_state, 5, &eval_func, &mut nodes_searched, &mut path);

        assert_eq!(score, 4);
        assert_eq!(nodes_searched, 129);
    }
}
