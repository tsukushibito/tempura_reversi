use reversi_core::{board::Board, Color, Move};

use crate::{GameState, SearchResult};

fn evaluate<B: Board>(state: &GameState<B>, color: Color) -> i32 {
    let black_count = state.board.black_count() as i32;
    let white_count = state.board.white_count() as i32;
    match color {
        Color::Black => black_count - white_count,
        Color::White => white_count - black_count,
    }
}

fn negamax<B: Board>(state: &GameState<B>, depth: usize) -> SearchResult {
    // Count the current node
    let mut nodes_searched = 1;

    // Get valid moves for the current player
    let valid_moves = state.board.get_valid_moves(state.player);

    // Check for terminal condition
    if depth == 0 {
        let score = evaluate(state, state.player);
        return SearchResult {
            best_move: None,
            path: Vec::new(),
            nodes_searched,
            score,
        };
    }

    if valid_moves.is_empty() {
        // No valid moves for current player
        // Check if opponent has valid moves
        let opponent_moves = state.board.get_valid_moves(state.player.opposite());
        if opponent_moves.is_empty() {
            // No valid moves for both players, game over
            let score = evaluate(state, state.player);
            return SearchResult {
                best_move: None,
                path: Vec::new(),
                nodes_searched,
                score,
            };
        } else {
            // Pass the turn to opponent
            let new_state = GameState {
                board: state.board.clone(),
                player: state.player.opposite(),
            };
            let result = negamax(&new_state, depth - 1);
            let score = -result.score;
            nodes_searched += result.nodes_searched;
            return SearchResult {
                best_move: None,
                path: result.path,
                nodes_searched,
                score,
            };
        }
    }

    // Initialize variables for tracking the best move and score
    let mut max_score = i32::MIN;
    let mut best_move = None;
    let mut best_path = Vec::new();

    // Iterate over all valid moves
    for mv_pos in valid_moves {
        // Clone the board and apply the move
        let mut new_board = state.board.clone();
        new_board.make_move(state.player, &mv_pos);

        // Create a new game state with the updated board and opponent's turn
        let new_state = GameState {
            board: new_board,
            player: state.player.opposite(),
        };

        // Recursively call negamax
        let result = negamax(&new_state, depth - 1);

        // Negate the score because the opponent's perspective is inverse
        let score = -result.score;

        nodes_searched += result.nodes_searched;

        // Update the best move if a higher score is found
        if score > max_score {
            max_score = score;
            best_move = Some(Move {
                position: Some(mv_pos),
                color: state.player,
            });
            best_path = vec![Move {
                position: Some(mv_pos),
                color: state.player,
            }];
            best_path.extend(result.path);
        }
    }

    // Return the search result with the best move and score
    SearchResult {
        best_move,
        path: best_path,
        nodes_searched,
        score: max_score,
    }
}
