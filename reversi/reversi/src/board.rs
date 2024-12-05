use crate::{BoardState, CellState, Color, Position};

pub const BOARD_SIZE: usize = 8;

pub trait CloneAsBoard {
    fn clone_as_board(&self) -> Box<dyn Board + Send>;
}

impl<T: Board + Send + Clone + 'static> CloneAsBoard for T {
    fn clone_as_board(&self) -> Box<dyn Board + Send> {
        Box::new(self.clone())
    }
}

pub trait Board: CloneAsBoard + std::fmt::Debug {
    fn clear(&mut self) {
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                self.set_disc(
                    &Position {
                        x: x as i8,
                        y: y as i8,
                    },
                    None,
                );
            }
        }
    }

    fn init(&mut self) {
        self.clear();

        self.set_disc(&Position::E4, Some(Color::Black));
        self.set_disc(&Position::D5, Some(Color::Black));
        self.set_disc(&Position::D4, Some(Color::White));
        self.set_disc(&Position::E5, Some(Color::White));
    }

    fn board_state(&self) -> BoardState {
        let mut board_state: BoardState = Default::default();
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                let index = y * BOARD_SIZE + x;
                let pos = Position {
                    x: x as i8,
                    y: y as i8,
                };
                board_state.cells[index] = self.get_disc(&pos).into();
            }
        }

        board_state
    }

    fn set_board_state(&mut self, board_state: &BoardState) {
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                let index = y * BOARD_SIZE + x;
                let pos = Position {
                    x: x as i8,
                    y: y as i8,
                };
                self.set_disc(&pos, board_state.cells[index].into());
            }
        }
    }

    fn discs(&self) -> Vec<Vec<Option<Color>>>;
    fn get_disc(&self, pos: &Position) -> Option<Color>;
    fn set_disc(&mut self, pos: &Position, color: Option<Color>);

    fn count_of(&self, color: Option<Color>) -> usize;

    fn black_count(&self) -> usize {
        self.count_of(Some(Color::Black))
    }

    fn white_count(&self) -> usize {
        self.count_of(Some(Color::White))
    }

    fn empty_count(&self) -> usize {
        self.count_of(None)
    }

    fn make_move(&mut self, color: Color, pos: &Position) -> bool;

    fn get_valid_moves(&self, color: Color) -> Vec<Position>;

    fn display(&self);
}
