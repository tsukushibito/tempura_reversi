use std::io::{self, Write};
use std::{sync::mpsc, thread};

use reversi::{
    ai::{ai_player::AiPlayer, evaluate, human_player::HumanPlayer, player::Player},
    bit_board::BitBoard,
    board::Board,
    game_play::{Game, GameEvent, GameState},
    Color,
};

fn main() {
    // チャネルの作成
    let (tx, rx) = mpsc::channel();

    // ボードの初期化
    let board = BitBoard::new();

    // プレイヤーの初期化
    let black_player: Box<dyn Player<BitBoard> + Send> = Box::new(HumanPlayer);
    // let black_player: Box<dyn Player<BitBoard> + Send> =
    //     Box::new(AiPlayer::new(evaluate::mobility_evaluate, Color::Black));
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
                    println!("Game Started!");
                    board.display();
                }
                GameEvent::MoveMade {
                    position,
                    color,
                    board,
                } => {
                    println!("{:?} make move with {:?} ", color, position);
                    board.display();
                }
                GameEvent::PlayerPassed { color, board } => {
                    println!("{:?} passed", color);
                    board.display();
                }
                GameEvent::GameOver {
                    black_score,
                    white_score,
                    winner,
                    board,
                } => {
                    println!(
                        "Game Over score: black {} - white {}",
                        black_score, white_score
                    );
                    match winner {
                        Some(color) => println!("{:?} wins", color),
                        None => println!("draw"),
                    }
                    board.display();
                    break;
                }
                GameEvent::GameReset { board } => {
                    println!("Reseted");
                    board.display();
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

fn select_player<B: Board + 'static>(
    color: Color,
    evaluate_fn: fn(&GameState<B>, Color) -> i32,
) -> Box<dyn Player<B>> {
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
