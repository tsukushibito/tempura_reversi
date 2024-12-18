use rayon::iter::{IntoParallelIterator, ParallelIterator};
use reversi::{Ai, BitBoard, Board, Color, Game, Negaalpha, Position, Searcher, TestEvaluator};

fn main() {
    let results: Vec<i32> = (0..10)
        .into_par_iter()
        .map(|_| {
            let mut ai_black = Ai::new();
            let mut ai_white = Ai::new();
            ai_white.searcher = Searcher::TestNegaalpha(Negaalpha::new(TestEvaluator::default()));

            // ゲームの初期化
            let mut game = Game::initial();

            // ゲームイベントの処理
            loop {
                if game.is_game_over() {
                    // println!("Game Over");
                    // game.board().display();
                    break;
                }

                // println!("Turn: {:?}", game.current_player());
                // game.board().display();

                let bit_board = BitBoard::from_board(game.board());
                match game.current_player() {
                    Color::Black => {
                        // let p = get_move_from_stdin(&bit_board, Color::Black);
                        let p = ai_black.decide_move(&bit_board, Color::Black);
                        let _ = game.progress(Color::Black, p.unwrap());
                    }
                    Color::White => {
                        let p = ai_white.decide_move(&bit_board, Color::White);
                        let _ = game.progress(Color::White, p.unwrap());
                    }
                }
            }

            match game.black_score().cmp(&game.white_score()) {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            }
        })
        .collect();

    println!("{:?}", results);

    let mut s: String = Default::default();
    std::io::stdin().read_line(&mut s).ok();
}

fn get_move_from_stdin(board: &BitBoard, color: Color) -> Option<Position> {
    loop {
        println!("Enter your move (e.g., D3): ");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match parse_position(&input) {
            Some(pos) => {
                if board.get_valid_moves(color).contains(&pos) {
                    return Some(pos);
                } else {
                    println!("Invalid move(): not a valid position. Try again.");
                }
            }
            None => println!("Invalid input format.({}) Please enter like D3.", input),
        }
    }
}

fn parse_position(input: &str) -> Option<Position> {
    let trimmed = input.trim().to_uppercase();
    if trimmed.len() < 2 {
        return None;
    }

    let chars: Vec<char> = trimmed.chars().collect();
    let col_char = chars[0];
    let row_str: String = chars[1..].iter().collect();

    let x = match col_char {
        'A'..='H' => (col_char as u8) - b'A',
        _ => return None,
    };

    let y = match row_str.parse::<u8>() {
        Ok(n) if (1..=8).contains(&n) => n - 1,
        _ => return None,
    };

    Some(Position::new(x as usize, y as usize))
}
