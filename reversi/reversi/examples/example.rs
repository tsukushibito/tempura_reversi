use std::io;
use std::{sync::mpsc, thread};

use reversi::game::GameCommand;
use reversi::{
    ai::{ai_player::AiPlayer, evaluate, human_player::HumanPlayer, player::Player},
    bit_board::BitBoard,
    game::{Game, GameEvent},
    Color,
};

fn main() {
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
    let mut s: String = Default::default();
    std::io::stdin().read_line(&mut s).ok();
}
