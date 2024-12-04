use std::sync::mpsc::{Receiver, Sender};

use crate::{bit_board::BitBoard, board::Board, Color, Position};

#[derive(Copy, Clone, Debug)]
pub enum CellState {
    Empty,
    Black,
    White,
}

#[derive(Debug)]
pub struct GameState {
    pub board: Box<dyn Board + Send>,
    pub current_player: Color,
    pub move_count: u32,
    pub is_game_over: bool,
}

impl GameState {
    pub fn new<T: Board + Send + Clone + 'static>(board: &T, player: Color) -> Self {
        Self {
            board: Box::new(board.clone()),
            current_player: player,
            move_count: 0,
            is_game_over: false,
        }
    }

    pub fn initial_state() -> Self {
        Self {
            board: Box::new(BitBoard::init_board()),
            current_player: Color::Black,
            move_count: 0,
            is_game_over: false,
        }
    }

    pub fn reset(&mut self) {
        self.board.init();
        self.current_player = Color::Black;
        self.move_count = 0;
        self.is_game_over = false;
    }

    pub fn get_current_players_valid_moves(&self) -> Vec<Position> {
        self.board.get_valid_moves(self.current_player)
    }

    pub fn switch_turn(&mut self) {
        self.current_player = self.current_player.opponent();
    }
}

impl Clone for GameState {
    fn clone(&self) -> Self {
        Self {
            board: self.board.clone_as_board(),
            current_player: self.current_player,
            move_count: self.move_count,
            is_game_over: self.is_game_over,
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
    Turn(GameState),
    GameOver(GameState),
}

#[derive(Debug, Clone)]
pub enum GameCommand {
    MakeMove { position: Position, player: Color },
    Reset,
    Exit,
}

/// ゲーム構造体
pub struct Game {
    state: GameState,
    event_sender: Sender<GameEvent>,
    command_receiver: Receiver<GameCommand>,
}

impl Game {
    pub fn new(event_sender: Sender<GameEvent>, command_receiver: Receiver<GameCommand>) -> Self {
        Game {
            state: GameState::initial_state(),
            event_sender,
            command_receiver,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            if !self.state.is_game_over {
                let valid_moves = self.state.get_current_players_valid_moves();
                if valid_moves.is_empty() {
                    // パスなのでプレイヤー交代
                    self.state.switch_turn();

                    let valid_moves = self.state.board.get_valid_moves(self.state.current_player);
                    if valid_moves.is_empty() {
                        // 双方パスなので終了
                        self.end_game()?;
                        continue;
                    }
                }

                // 状態通知
                self.event_sender
                    .send(GameEvent::Turn(self.state.clone()))
                    .map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;
            }

            let command = self
                .command_receiver
                .recv()
                .map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;

            match command {
                GameCommand::MakeMove { position, player } => {
                    if player != self.state.current_player {
                        continue;
                    }
                    let mut board = self.state.board.clone_as_board();
                    let success = board.make_move(player, &position);
                    if success {
                        self.state.switch_turn();
                        self.state.board = board;
                    }
                }
                GameCommand::Reset => {
                    self.state.reset();
                }
                GameCommand::Exit => break,
            }
        }

        Ok(())
    }

    /// ゲームを終了し、結果を送信する関数
    fn end_game(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let game_over_event = GameEvent::GameOver(self.state.clone());
        self.event_sender
            .send(game_over_event)
            .map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use crate::{
        ai::{ai_player::AiPlayer, evaluate, human_player::HumanPlayer, player::Player},
        Color,
    };

    use super::{Game, GameCommand, GameEvent};

    #[test]
    fn test_game_play() {
        // プレイヤーの初期化
        let mut black_player = HumanPlayer {};
        let mut white_player = AiPlayer::new(evaluate::mobility_evaluate, Color::White);

        // ゲームの初期化
        let (event_sender, event_receiver) = std::sync::mpsc::channel();
        let (command_sender, command_receiver) = std::sync::mpsc::channel();
        let mut game = Game::new(event_sender, command_receiver);

        // ゲームを別スレッドで実行
        let thread_handle = thread::spawn(move || {
            let _ = game.run();
        });

        // ゲームイベントの処理
        loop {
            let r = event_receiver.recv();
            match r {
                Ok(event) => match event {
                    GameEvent::Turn(game_state) => {
                        println!("Turn: {:?}", game_state.current_player);
                        game_state.board.display();

                        match game_state.current_player {
                            Color::Black => {
                                let p = black_player.get_move(&game_state).unwrap();
                                command_sender
                                    .send(GameCommand::MakeMove {
                                        position: p,
                                        player: Color::Black,
                                    })
                                    .unwrap();
                            }
                            Color::White => {
                                let p = white_player.get_move(&game_state).unwrap();
                                command_sender
                                    .send(GameCommand::MakeMove {
                                        position: p,
                                        player: Color::White,
                                    })
                                    .unwrap();
                            }
                        }
                    }
                    GameEvent::GameOver(game_state) => {
                        command_sender.send(GameCommand::Exit).unwrap();
                        println!("Game Over");
                        game_state.board.display();
                        break;
                    }
                },
                Err(error) => {
                    println!("{}", error);
                }
            }
        }

        let _ = thread_handle.join();
    }
}
