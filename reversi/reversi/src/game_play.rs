use std::sync::mpsc::Sender;

use crate::{
    ai::player::Player,
    bit_board::BitBoard,
    board::{Board, BOARD_SIZE},
    Color, Position,
};

#[derive(Copy, Clone, Debug)]
pub enum CellState {
    Empty,
    Black,
    White,
}

pub type BoardState = [CellState; BOARD_SIZE * BOARD_SIZE];

pub fn board_state_to_bit_board(board: &BoardState) -> BitBoard {
    let mut bit_board = BitBoard::default();

    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            let index = y * BOARD_SIZE + x;
            let color = match board[index] {
                CellState::Empty => None,
                CellState::Black => Some(Color::Black),
                CellState::White => Some(Color::White),
            };
            bit_board.set_disc(
                &Position {
                    x: x as i8,
                    y: y as i8,
                },
                color,
            );
        }
    }

    bit_board
}

pub fn bit_board_to_board_state(bit_board: &BitBoard) -> BoardState {
    let mut board_state = [CellState::Empty; BOARD_SIZE * BOARD_SIZE];

    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            let index = y * BOARD_SIZE + x;
            let cell = match bit_board.get_disc(&Position {
                x: x as i8,
                y: y as i8,
            }) {
                Some(Color::Black) => CellState::Black,
                Some(Color::White) => CellState::White,
                None => CellState::Empty,
            };
            board_state[index] = cell;
        }
    }

    board_state
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub board: BoardState,
    pub player: Color,
}

impl GameState {
    pub fn new(board: BoardState, player: Color) -> Self {
        GameState { board, player }
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
            state: GameState::new(bit_board_to_board_state(&BitBoard::new()), Color::Black),
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
            let mut bit_board = board_state_to_bit_board(&self.state.board);
            let valid_moves = bit_board.get_valid_moves(current_player);
            if valid_moves.is_empty() {
                // プレイヤーがパスする必要がある
                let _ = self.event_sender.send(GameEvent::PlayerPassed {
                    state: self.state.clone(),
                });
                // プレイヤーを交代
                self.state.player = current_player.opponent();

                // 両プレイヤーがパスした場合、ゲーム終了
                if bit_board.get_valid_moves(self.state.player).is_empty() {
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
                    let success = bit_board.make_move(current_player, &pos);
                    if success {
                        self.state.board = bit_board_to_board_state(&bit_board);
                        // MoveMade イベントを送信
                        let _ = self.event_sender.send(GameEvent::MoveMade {
                            position: pos,
                            color: current_player,
                            state: GameState {
                                board: bit_board_to_board_state(&bit_board),
                                player: current_player,
                            },
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
                        state: GameState {
                            board: bit_board_to_board_state(&bit_board),
                            player: current_player,
                        },
                    });
                    self.state.player = current_player.opponent();

                    // 両プレイヤーがパスした場合、ゲーム終了
                    if bit_board.get_valid_moves(self.state.player).is_empty() {
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
        let bit_board = board_state_to_bit_board(&self.state.board);
        let black_score = bit_board.black_count();
        let white_score = bit_board.white_count();
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
        self.state = GameState::new(bit_board_to_board_state(&BitBoard::new()), Color::Black);
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
        board::Board,
        Color,
    };

    use super::{board_state_to_bit_board, BoardState, Game, GameEvent};

    fn display_board(board: &BoardState) {
        board_state_to_bit_board(board).display();
    }

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
                        display_board(&state.board);
                    }
                    GameEvent::MoveMade {
                        position,
                        color,
                        state,
                    } => {
                        println!("{:?} プレイヤーが {:?} に手を打ちました。", color, position);
                        display_board(&state.board);
                    }
                    GameEvent::PlayerPassed { state } => {
                        println!("{:?} プレイヤーはパスしました。", state.player);
                        display_board(&state.board);
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
                        display_board(&state.board);
                        break;
                    }
                    GameEvent::GameReset { state } => {
                        println!("ゲームがリセットされました。");
                        display_board(&state.board);
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
