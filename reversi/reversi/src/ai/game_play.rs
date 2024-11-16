use crate::{board::Board, Color};

use super::{player::Player, GameState};

pub struct Game<B: Board> {
    board: B,
    black_player: Box<dyn Player<B>>,
    white_player: Box<dyn Player<B>>,
}

impl<B: Board> Game<B> {
    /// 新しいゲームを初期化
    pub fn new(
        board: B,
        black_player: Box<dyn Player<B>>,
        white_player: Box<dyn Player<B>>,
    ) -> Self {
        Game {
            board,
            black_player,
            white_player,
        }
    }

    /// ゲームを開始し、プレイを管理する関数
    pub fn play(&mut self) {
        let mut state = GameState::new(self.board.clone(), Color::Black);

        loop {
            println!("Current board:");
            state.board.display();
            println!(
                "Black: {}, White: {}",
                state.board.black_count(),
                state.board.white_count()
            );

            let current_player = state.player;

            println!(
                "{}'s turn.",
                match current_player {
                    Color::Black => "Black",
                    Color::White => "White",
                }
            );

            // 現在のプレイヤーから手を取得
            let move_pos = match current_player {
                Color::Black => self.black_player.get_move(&state),
                Color::White => self.white_player.get_move(&state),
            };

            match move_pos {
                Some(pos) => {
                    let success = state.board.make_move(current_player, &pos);
                    if success {
                        println!(
                            "{} played at {}{}",
                            match current_player {
                                Color::Black => "Black",
                                Color::White => "White",
                            },
                            (pos.x as u8 + b'A') as char,
                            pos.y + 1
                        );
                    } else {
                        println!("Move failed: position not valid.");
                        continue;
                    }
                }
                None => {
                    println!(
                        "No valid moves available for {}.",
                        match current_player {
                            Color::Black => "Black",
                            Color::White => "White",
                        }
                    );
                }
            }

            // ゲーム終了判定
            let black_moves = state.board.get_valid_moves(Color::Black);
            let white_moves = state.board.get_valid_moves(Color::White);

            if black_moves.is_empty() && white_moves.is_empty() {
                println!("Game over.");
                println!(
                    "Final score - Black: {}, White: {}",
                    state.board.black_count(),
                    state.board.white_count()
                );
                if state.board.black_count() > state.board.white_count() {
                    println!("Black wins!");
                } else if state.board.white_count() > state.board.black_count() {
                    println!("White wins!");
                } else {
                    println!("It's a draw!");
                }
                break;
            }

            // プレイヤーを交代
            state.player = state.player.opponent();
        }
    }
}
