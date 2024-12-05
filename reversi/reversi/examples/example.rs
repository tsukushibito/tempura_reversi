use reversi::{
    ai::{ai_player::AiPlayer, evaluate, human_player::HumanPlayer, player::Player},
    bit_board::BitBoard,
    game::Game,
    Color,
};

fn main() {
    // プレイヤーの初期化
    // let mut black_player = HumanPlayer {};
    let mut black_player = AiPlayer::new(evaluate::mobility_evaluate, Color::Black);
    let mut white_player = AiPlayer::new(evaluate::mobility_evaluate, Color::White);

    // ゲームの初期化
    let mut game = Game::initial();

    // ゲームイベントの処理
    loop {
        if game.is_game_over() {
            println!("Game Over");
            game.board().display();
            break;
        }

        println!("Turn: {:?}", game.current_player());
        game.board().display();

        let bit_board = BitBoard::from_board(game.board());
        match game.current_player() {
            Color::Black => {
                let p = black_player.get_move(&bit_board, Color::Black);
                let _ = game.progress(Color::Black, p.unwrap());
            }
            Color::White => {
                let p = white_player.get_move(&bit_board, Color::White);
                let _ = game.progress(Color::White, p.unwrap());
            }
        }
    }

    let mut s: String = Default::default();
    std::io::stdin().read_line(&mut s).ok();
}
