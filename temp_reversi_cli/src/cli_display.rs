use temp_reversi_core::{Board, Game, Player};

pub fn cli_display(game: &Game<impl Board>) {
    if game.is_game_over() {
        println!("Game over!");
        println!("Board:\n{}", game.board_state());
        let (final_black_score, final_white_score) = game.current_score();
        println!(
            "Final Score - Black: {}, White: {}",
            final_black_score, final_white_score
        );
        match game.winner().unwrap() {
            Some(Player::Black) => println!("Winner: Black"),
            Some(Player::White) => println!("Winner: White"),
            None => println!("It's a draw!"),
        }
    } else {
        println!("Board:\n{}", game.board_state());
        let (black_score, white_score) = game.current_score();
        println!(
            "Player: {}, Score - Black: {}, White: {}",
            match game.current_player() {
                Player::Black => "Black",
                Player::White => "White",
            },
            black_score,
            white_score
        );

        let valid_moves = game
            .valid_moves()
            .iter()
            .map(|pos| format!("{}", pos))
            .collect::<Vec<String>>()
            .join(", ");
        println!("Valid moves: [{}]", valid_moves);
    }
}
