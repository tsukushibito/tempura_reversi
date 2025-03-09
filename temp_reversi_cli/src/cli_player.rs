use std::str::FromStr;
use temp_reversi_core::{Game, MoveDecider, Position};

pub struct CliPlayer;

impl MoveDecider for CliPlayer {
    fn select_move(&mut self, game: &Game) -> Option<Position> {
        println!("Enter your move (e.g., A1):");
        let mut position = None;
        loop {
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");
            let input = input.trim();

            match Position::from_str(input) {
                Ok(p) => {
                    if !game.valid_moves().iter().any(|m| *m == p) {
                        println!("Invalid position.");
                        continue;
                    }
                    position = Some(p);
                    break;
                }
                Err(err) => {
                    println!("Error: {}", err);
                    break;
                }
            }
        }

        position
    }
}
