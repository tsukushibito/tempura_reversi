use std::io::{self, Write};

use crate::{
    board::Board,
    game_play::{board_state_to_bit_board, GameState},
    Position,
};

use super::player::Player;

pub struct HumanPlayer;

impl Player for HumanPlayer {
    fn get_move(&mut self, state: &GameState) -> Option<Position> {
        loop {
            println!("Enter your move (e.g., D3): ");
            std::io::stdout().flush();
            let mut input = String::new();
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let bit_board = board_state_to_bit_board(&state.board);

            match parse_position(&input) {
                Some(pos) => {
                    if bit_board.get_valid_moves(state.player).contains(&pos) {
                        return Some(pos);
                    } else {
                        println!("Invalid move: not a valid position. Try again.");
                    }
                }
                None => println!("Invalid input format. Please enter like D3."),
            }
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

    Some(Position {
        x: x as i8,
        y: y as i8,
    })
}
