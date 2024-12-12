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
                self.set_cell_state(
                    &Position {
                        x: x as u8,
                        y: y as u8,
                    },
                    CellState::Empty,
                );
            }
        }
    }

    fn init(&mut self) {
        self.clear();

        self.set_cell_state(&Position::E4, CellState::Disc(Color::Black));
        self.set_cell_state(&Position::D5, CellState::Disc(Color::Black));
        self.set_cell_state(&Position::D4, CellState::Disc(Color::White));
        self.set_cell_state(&Position::E5, CellState::Disc(Color::White));
    }

    fn board_state(&self) -> BoardState {
        BoardState {
            cells: self.cell_states(),
        }
    }

    fn set_board_state(&mut self, board_state: &BoardState) {
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                let index = y * BOARD_SIZE + x;
                let pos = Position::new(x, y);
                self.set_cell_state(&pos, board_state.cells[index]);
            }
        }
    }

    fn cell_states(&self) -> [CellState; BOARD_SIZE * BOARD_SIZE];
    fn get_cell_state(&self, pos: &Position) -> CellState;
    fn set_cell_state(&mut self, pos: &Position, cell: CellState);

    fn count_of(&self, color: CellState) -> usize;

    fn black_count(&self) -> usize {
        self.count_of(CellState::Disc(Color::Black))
    }

    fn white_count(&self) -> usize {
        self.count_of(CellState::Disc(Color::White))
    }

    fn empty_count(&self) -> usize {
        self.count_of(CellState::Empty)
    }

    fn make_move(&mut self, color: Color, pos: &Position) -> bool;

    fn get_valid_moves(&self, color: Color) -> Vec<Position>;

    fn display(&self);
}
