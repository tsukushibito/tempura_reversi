use std::sync::mpsc::Sender;

use crate::{board::Board, Color, Position};

use super::{player::Player, GameState};

/// ゲーム結果を表す列挙型
#[derive(Debug, Clone)]
pub enum GameResult {
    BlackWins(usize, usize),
    WhiteWins(usize, usize),
    Draw(usize, usize),
}

#[derive(Debug, Clone)]
pub enum GameEvent<B: Board> {
    GameStarted {
        current_player: Color,
        board: B,
    },
    MoveMade {
        position: Position,
        color: Color,
        board: B,
    },
    PlayerPassed {
        color: Color,
        board: B,
    },
    GameOver {
        black_score: usize,
        white_score: usize,
        winner: Option<Color>,
        board: B,
    },
    GameReset {
        board: B,
    },
}

/// ゲーム構造体
pub struct Game<B: Board> {
    black_player: Box<dyn Player<B> + Send>,
    white_player: Box<dyn Player<B> + Send>,
    state: GameState<B>,
    event_sender: Sender<GameEvent<B>>, // イベント送信用チャネル
}

impl<B: Board + Send + 'static> Game<B> {
    /// 新しいゲームを初期化
    pub fn new(
        board: B,
        black_player: Box<dyn Player<B> + Send>,
        white_player: Box<dyn Player<B> + Send>,
        sender: Sender<GameEvent<B>>,
    ) -> Self {
        Game {
            black_player,
            white_player,
            state: GameState::new(board, Color::Black),
            event_sender: sender,
        }
    }

    /// ゲームをプレイする関数
    pub fn play(mut self) {
        // ゲーム開始イベントを送信
        let _ = self.event_sender.send(GameEvent::GameStarted {
            current_player: self.state.player,
            board: self.state.board.clone(),
        });

        loop {
            // 現在のプレイヤー
            let current_player = self.state.player;

            // 現在のプレイヤーが有効な手を持っているか確認
            let valid_moves = self.state.board.get_valid_moves(current_player);
            if valid_moves.is_empty() {
                // プレイヤーがパスする必要がある
                let _ = self.event_sender.send(GameEvent::PlayerPassed {
                    color: current_player,
                    board: self.state.board.clone(),
                });
                // プレイヤーを交代
                self.state.player = current_player.opponent();

                // 両プレイヤーがパスした場合、ゲーム終了
                if self
                    .state
                    .board
                    .get_valid_moves(self.state.player)
                    .is_empty()
                {
                    self.end_game();
                    break;
                }
                continue;
            }

            // プレイヤーから手を取得
            let move_pos = match current_player {
                Color::Black => self.black_player.get_move(&self.state),
                Color::White => self.white_player.get_move(&self.state),
            };

            match move_pos {
                Some(pos) => {
                    let success = self.state.board.make_move(current_player, &pos);
                    if success {
                        // MoveMade イベントを送信
                        let _ = self.event_sender.send(GameEvent::MoveMade {
                            position: pos,
                            color: current_player,
                            board: self.state.board.clone(),
                        });
                    } else {
                        // 無効な手の場合、再試行（ここではスキップ）
                        // 必要に応じてエラーハンドリングを追加
                        continue;
                    }
                }
                None => {
                    // プレイヤーが手を打てない場合、パス
                    let _ = self.event_sender.send(GameEvent::PlayerPassed {
                        color: current_player,
                        board: self.state.board.clone(),
                    });
                    self.state.player = current_player.opponent();

                    // 両プレイヤーがパスした場合、ゲーム終了
                    if self
                        .state
                        .board
                        .get_valid_moves(self.state.player)
                        .is_empty()
                    {
                        self.end_game();
                        break;
                    }
                }
            }

            // プレイヤーを交代
            self.state.player = self.state.player.opponent();
        }
    }

    /// ゲームを終了し、結果を送信する関数
    fn end_game(&mut self) {
        let black_score = self.state.board.black_count();
        let white_score = self.state.board.white_count();
        let winner = match black_score.cmp(&white_score) {
            std::cmp::Ordering::Greater => Some(Color::Black),
            std::cmp::Ordering::Less => Some(Color::White),
            std::cmp::Ordering::Equal => None,
        };

        let game_over_event = GameEvent::GameOver {
            black_score,
            white_score,
            winner,
            board: self.state.board.clone(),
        };

        let _ = self.event_sender.send(game_over_event);
    }

    /// ゲームをリセットする関数（必要に応じて追加）
    pub fn reset(&mut self, new_board: B) {
        self.state = GameState::new(new_board.clone(), Color::Black);
        let _ = self
            .event_sender
            .send(GameEvent::GameReset { board: new_board });
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::mpsc, thread};

    use crate::{
        ai::{ai_player::AiPlayer, evaluate, human_player::HumanPlayer, player::Player},
        bit_board::BitBoard,
        board::Board,
        Color,
    };

    use super::{Game, GameEvent};

    #[test]
    fn test_game_play() {
        // チャネルの作成
        let (tx, rx) = mpsc::channel();

        // ボードの初期化
        let board = BitBoard::new();

        // プレイヤーの初期化
        // let black_player: Box<dyn Player<BitBoard> + Send> = Box::new(HumanPlayer);
        let black_player: Box<dyn Player<BitBoard> + Send> =
            Box::new(AiPlayer::new(evaluate::mobility_evaluate, Color::Black));
        let white_player: Box<dyn Player<BitBoard> + Send> =
            Box::new(AiPlayer::new(evaluate::mobility_evaluate, Color::White));

        // ゲームの初期化
        let game = Game::new(board, black_player, white_player, tx);

        // ゲームを別スレッドで実行
        thread::spawn(move || {
            game.play();
        });

        // ゲームイベントの処理
        loop {
            match rx.recv() {
                Ok(event) => match event {
                    GameEvent::GameStarted {
                        current_player: _,
                        board,
                    } => {
                        println!("ゲームが開始されました。");
                        board.display();
                    }
                    GameEvent::MoveMade {
                        position,
                        color,
                        board,
                    } => {
                        println!("{:?} プレイヤーが {:?} に手を打ちました。", color, position);
                        board.display();
                    }
                    GameEvent::PlayerPassed { color, board } => {
                        println!("{:?} プレイヤーはパスしました。", color);
                        board.display();
                    }
                    GameEvent::GameOver {
                        black_score,
                        white_score,
                        winner,
                        board,
                    } => {
                        println!(
                            "ゲームが終了しました。スコア: 黒 {} - 白 {}",
                            black_score, white_score
                        );
                        match winner {
                            Some(color) => println!("{:?} プレイヤーの勝利です！", color),
                            None => println!("引き分けです！"),
                        }
                        board.display();
                        break;
                    }
                    GameEvent::GameReset { board } => {
                        println!("ゲームがリセットされました。");
                        board.display();
                    }
                },
                Err(_) => {
                    println!("ゲームスレッドが終了しました。");
                    break;
                }
            }
        }
    }
}
