use crate::board::{Board, Color, BOARD_SIZE};

const EMPTY: u8 = 0;
const BLACK: u8 = 1;
const WHITE: u8 = 2;

#[derive(Debug, Clone)]
pub struct ArrayBoard {
    pub discs: [u8; BOARD_SIZE * BOARD_SIZE],
}

impl Board for ArrayBoard {
    fn discs(&self) -> Vec<Vec<Option<Color>>> {
        let mut discs = Vec::new();
        for y in 0..BOARD_SIZE {
            let mut row = Vec::new();
            for x in 0..BOARD_SIZE {
                let index = x + y * BOARD_SIZE;
                let color = match self.discs[index] {
                    EMPTY => None,
                    BLACK => Some(Color::Black),
                    WHITE => Some(Color::White),
                    _ => None,
                };
                row.push(color);
            }
            discs.push(row);
        }

        discs
    }

    fn count_of(&self, color: Option<Color>) -> usize {
        let c = match color {
            None => EMPTY,
            Some(col) => match col {
                Color::Black => BLACK,
                Color::White => WHITE,
            },
        };
        let mut count = 0;
        for disc in self.discs {
            if disc == c {
                count += 1;
            }
        }
        count
    }

    fn make_move(&mut self, color: Color, pos: &crate::board::Position) -> bool {
        todo!()
    }

    fn get_valid_moves(&self, color: Color) -> Vec<crate::board::Position> {
        todo!()
    }

    fn display(&self) {
        todo!()
    }
}
