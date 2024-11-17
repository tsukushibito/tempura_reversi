use std::io::{self, Write};

use reversi::{
    ai::{
        ai_player::AIPlayer, game_play::Game, human_player::HumanPlayer, player::Player, GameState,
    },
    bit_board::BitBoard,
    board::Board,
    Color,
};

fn main() {
    // 盤面評価関数を定義または参照
    fn evaluate_fn<B: Board>(state: &GameState<B>, color: Color) -> i32 {
        // 例: 石の数の差を評価値とする
        if color == Color::Black {
            state.board.black_count() as i32 - state.board.white_count() as i32
        } else {
            state.board.white_count() as i32 - state.board.black_count() as i32
        }
    }

    // プレイヤーの選択
    println!("Select Black player type (Human: h, AI: a): ");
    let black_player = select_player(Color::Black, evaluate_fn);

    println!("Select White player type (Human: h, AI: a): ");
    let white_player = select_player(Color::White, evaluate_fn);

    // ゲームを初期化し、プレイを開始
    let board = BitBoard::new();
    let mut game = Game::new(board, black_player, white_player);
    game.play();
}

fn select_player<B: Board + 'static>(
    color: Color,
    evaluate_fn: fn(&GameState<B>, Color) -> i32,
) -> Box<dyn Player<B>> {
    loop {
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().to_lowercase().as_str() {
            "human" | "h" => {
                return Box::new(HumanPlayer);
            }
            "ai" | "a" => {
                return Box::new(AIPlayer::new(evaluate_fn, color));
            }
            _ => {
                println!("Invalid input. Please enter 'h' for Human or 'a' for AI.");
            }
        }
    }
}
