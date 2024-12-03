use std::sync::mpsc::Sender;

use crate::{ai::player::Player, bit_board::BitBoard, board::Board, Color, Position};

#[derive(Copy, Clone, Debug)]
pub enum CellState {
    Empty,
    Black,
    White,
}

#[derive(Debug)]
pub struct GameState {
    pub board: Box<dyn Board + Send>,
    pub player: Color,
}

impl GameState {
    pub fn new<T: Board + Send + Clone + 'static>(board: &T, player: Color) -> Self {
        Self {
            board: Box::new(board.clone()),
            player,
        }
    }

    pub fn init() -> Self {
        Self {
            board: Box::new(BitBoard::init_board()),
            player: Color::Black,
        }
    }
}

impl Clone for GameState {
    fn clone(&self) -> Self {
        Self {
            board: self.board.clone_as_board(),
            player: self.player,
        }
    }
}

/// ゲーム結果を表す列挙型
#[derive(Debug, Clone)]
pub enum GameResult {
    BlackWins(usize, usize),
    WhiteWins(usize, usize),
    Draw(usize, usize),
}

#[derive(Debug, Clone)]
pub enum GameEvent {
    GameStarted {
        state: GameState,
    },
    MoveMade {
        position: Position,
        color: Color,
        state: GameState,
    },
    PlayerPassed {
        state: GameState,
    },
    GameOver {
        black_score: usize,
        white_score: usize,
        winner: Option<Color>,
        state: GameState,
    },
    GameReset {
        state: GameState,
    },
}

/// ゲーム構造体
pub struct Game {
    black_player: Box<dyn Player + Send>,
    white_player: Box<dyn Player + Send>,
    state: GameState,
    event_sender: Sender<GameEvent>, // イベント送信用チャネル
}

impl Game {
    /// 新しいゲームを初期化
    pub fn new(
        black_player: Box<dyn Player + Send>,
        white_player: Box<dyn Player + Send>,
        sender: Sender<GameEvent>,
    ) -> Self {
        Game {
            black_player,
            white_player,
            state: GameState::init(),
            event_sender: sender,
        }
    }

    /// ゲームをプレイする関数
    pub fn play(mut self) {
        // ゲーム開始イベントを送信
        let _ = self.event_sender.send(GameEvent::GameStarted {
            state: self.state.clone(),
        });

        loop {
            // 現在のプレイヤー
            let current_player = self.state.player;

            // 現在のプレイヤーが有効な手を持っているか確認
            let board = &self.state.board;
            let valid_moves = board.get_valid_moves(current_player);
            if valid_moves.is_empty() {
                // プレイヤーがパスする必要がある
                let _ = self.event_sender.send(GameEvent::PlayerPassed {
                    state: self.state.clone(),
                });
                // プレイヤーを交代
                self.state.player = current_player.opponent();

                // 両プレイヤーがパスした場合、ゲーム終了
                if board.get_valid_moves(self.state.player).is_empty() {
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
                    let mut new_board = board.clone_as_board();
                    let success = new_board.make_move(current_player, &pos);
                    if success {
                        self.state.board = new_board;
                        // MoveMade イベントを送信
                        let _ = self.event_sender.send(GameEvent::MoveMade {
                            position: pos,
                            color: current_player,
                            state: self.state.clone(),
                        });
                    } else {
                        // 無効な手の場合、再試行（ここではスキップ）
                        // 必要に応じてエラーハンドリングを追加
                        continue;
                    }
                }
                None => {
                    // プレイヤーが手を打てない場合、パス
                    self.state.player = current_player.opponent();

                    let _ = self.event_sender.send(GameEvent::PlayerPassed {
                        state: self.state.clone(),
                    });

                    // 両プレイヤーがパスした場合、ゲーム終了
                    if board.get_valid_moves(self.state.player).is_empty() {
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
        let board = &self.state.board;
        let black_score = board.black_count();
        let white_score = board.white_count();
        let winner = match black_score.cmp(&white_score) {
            std::cmp::Ordering::Greater => Some(Color::Black),
            std::cmp::Ordering::Less => Some(Color::White),
            std::cmp::Ordering::Equal => None,
        };

        let game_over_event = GameEvent::GameOver {
            black_score,
            white_score,
            winner,
            state: self.state.clone(),
        };

        let _ = self.event_sender.send(game_over_event);
    }

    /// ゲームをリセットする関数（必要に応じて追加）
    pub fn reset(&mut self) {
        self.state = GameState::init();
        let _ = self.event_sender.send(GameEvent::GameReset {
            state: self.state.clone(),
        });
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::mpsc, thread};

    use crate::{
        ai::{ai_player::AiPlayer, evaluate, human_player::HumanPlayer, player::Player},
        Color,
    };

    use super::{Game, GameEvent};

    #[test]
    fn test_game_play() {
        // チャネルの作成
        let (tx, rx) = mpsc::channel();

        // プレイヤーの初期化
        let black_player: Box<dyn Player + Send> = Box::new(HumanPlayer);
        // let black_player: Box<dyn Player<BitBoard> + Send> =
        //     Box::new(AiPlayer::new(evaluate::mobility_evaluate, Color::Black));
        let white_player: Box<dyn Player + Send> =
            Box::new(AiPlayer::new(evaluate::mobility_evaluate, Color::White));

        // ゲームの初期化
        let game = Game::new(black_player, white_player, tx);

        // ゲームを別スレッドで実行
        thread::spawn(move || {
            game.play();
        });

        // ゲームイベントの処理
        loop {
            match rx.recv() {
                Ok(event) => match event {
                    GameEvent::GameStarted { state } => {
                        println!("ゲームが開始されました。");
                        state.board.display();
                    }
                    GameEvent::MoveMade {
                        position,
                        color,
                        state,
                    } => {
                        println!("{:?} プレイヤーが {:?} に手を打ちました。", color, position);
                        state.board.display();
                    }
                    GameEvent::PlayerPassed { state } => {
                        println!("{:?} プレイヤーはパスしました。", state.player);
                        state.board.display();
                    }
                    GameEvent::GameOver {
                        black_score,
                        white_score,
                        winner,
                        state,
                    } => {
                        println!(
                            "ゲームが終了しました。スコア: 黒 {} - 白 {}",
                            black_score, white_score
                        );
                        match winner {
                            Some(color) => println!("{:?} プレイヤーの勝利です！", color),
                            None => println!("引き分けです！"),
                        }
                        state.board.display();
                        break;
                    }
                    GameEvent::GameReset { state } => {
                        println!("ゲームがリセットされました。");
                        state.board.display();
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
