use std::io;
use std::{sync::mpsc, thread};

use reversi::{
    ai::{ai_player::AiPlayer, evaluate, human_player::HumanPlayer, player::Player},
    bit_board::BitBoard,
    game::{Game, GameEvent},
    Color,
};

fn main() {
    // チャネルの作成
    let (tx, rx) = mpsc::channel();

    // プレイヤーの初期化
    // let black_player: Box<dyn Player + Send> = Box::new(HumanPlayer);
    let black_player: Box<dyn Player + Send> =
        Box::new(AiPlayer::new(evaluate::mobility_evaluate, Color::Black));
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
                    println!("Game Started!");
                    state.board.display();
                }
                GameEvent::MoveMade {
                    position,
                    color,
                    state,
                } => {
                    println!("{:?} make move with {:?} ", color, position);
                    state.board.display();
                }
                GameEvent::PlayerPassed { state } => {
                    println!("{:?} passed", state.player);
                    state.board.display();
                }
                GameEvent::GameOver {
                    black_score,
                    white_score,
                    winner,
                    state,
                } => {
                    println!(
                        "Game Over score: black {} - white {}",
                        black_score, white_score
                    );
                    match winner {
                        Some(color) => println!("{:?} wins", color),
                        None => println!("draw"),
                    }
                    state.board.display();
                    break;
                }
                GameEvent::GameReset { state } => {
                    println!("Reseted");
                    state.board.display();
                }
            },
            Err(_) => {
                println!("Game thread exited");
                break;
            }
        }
    }
    let mut s: String = Default::default();
    std::io::stdin().read_line(&mut s).ok();
}

fn select_player(color: Color, evaluate_fn: fn(&BitBoard, Color) -> i32) -> Box<dyn Player> {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().to_lowercase().as_str() {
            "human" | "h" => {
                return Box::new(HumanPlayer);
            }
            "ai" | "a" => {
                return Box::new(AiPlayer::new(evaluate_fn, color));
            }
            _ => {
                println!("Invalid input. Please enter 'h' for Human or 'a' for AI.");
            }
        }
    }
}
