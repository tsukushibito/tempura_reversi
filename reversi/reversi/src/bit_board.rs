use crate::{
    board::{Board, BOARD_SIZE},
    CellState, Color, Direction, Position,
};

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq)]
pub struct BitBoard {
    pub black: u64,
    pub white: u64,
}

const MASK_EXCLUDE_A_FILE: u64 = 0xfefefefefefefefeu64;
const MASK_EXCLUDE_H_FILE: u64 = 0x7f7f7f7f7f7f7f7fu64;
const MASK_EXCLUDE_FIRST_RANK: u64 = 0xffffffffffffff00u64;
const MASK_EXCLUDE_LAST_RANK: u64 = 0x00ffffffffffffffu64;
const MASK_FOR_HORIZONTAL: u64 = MASK_EXCLUDE_A_FILE & MASK_EXCLUDE_H_FILE;
const MASK_FOR_VERTICAL: u64 = MASK_EXCLUDE_FIRST_RANK & MASK_EXCLUDE_LAST_RANK;
const MASK_FOR_DIAGONAL: u64 = MASK_FOR_HORIZONTAL & MASK_FOR_VERTICAL;

fn get_shift_and_mask_for_valid_moves(dir: Direction) -> (i32, u64) {
    match dir {
        Direction::East => (1, MASK_FOR_HORIZONTAL),
        Direction::West => (-1, MASK_FOR_HORIZONTAL),
        Direction::South => (8, MASK_FOR_VERTICAL),
        Direction::North => (-8, MASK_FOR_VERTICAL),
        Direction::SouthEast => (9, MASK_FOR_DIAGONAL),
        Direction::SouthWest => (7, MASK_FOR_DIAGONAL),
        Direction::NorthEast => (-7, MASK_FOR_DIAGONAL),
        Direction::NorthWest => (-9, MASK_FOR_DIAGONAL),
    }
}

fn get_shift_and_mask_for_flips(dir: Direction) -> (i32, u64) {
    match dir {
        Direction::East => (1, MASK_EXCLUDE_A_FILE),
        Direction::West => (-1, MASK_EXCLUDE_H_FILE),
        Direction::South => (8, MASK_EXCLUDE_FIRST_RANK),
        Direction::North => (-8, MASK_EXCLUDE_LAST_RANK),
        Direction::SouthEast => (9, MASK_EXCLUDE_A_FILE & MASK_EXCLUDE_FIRST_RANK),
        Direction::SouthWest => (7, MASK_EXCLUDE_H_FILE & MASK_EXCLUDE_FIRST_RANK),
        Direction::NorthEast => (-7, MASK_EXCLUDE_A_FILE & MASK_EXCLUDE_LAST_RANK),
        Direction::NorthWest => (-9, MASK_EXCLUDE_H_FILE & MASK_EXCLUDE_LAST_RANK),
    }
}

fn shift_bits(bits: u64, shift_amount: i32) -> u64 {
    if shift_amount >= 0 {
        bits << shift_amount
    } else {
        bits >> -shift_amount
    }
}

fn get_valid_moves_bits(player_bits: u64, opponent_bits: u64) -> u64 {
    let empty = !(player_bits | opponent_bits);
    let mut valid_moves = 0u64;

    for dir in Direction::DIRECTIONS {
        let (shift_amount, mask) = get_shift_and_mask_for_valid_moves(dir);
        let watcher = opponent_bits & mask;
        let mut tmp = shift_bits(player_bits, shift_amount) & watcher;

        for _i in 0..6 {
            tmp |= shift_bits(tmp, shift_amount) & watcher;
        }

        valid_moves |= shift_bits(tmp, shift_amount) & empty;
    }

    valid_moves
}

fn get_flips_bits(move_bit: u64, player_bits: u64, opponent_bits: u64) -> u64 {
    let mut flips = 0u64;

    for dir in Direction::DIRECTIONS {
        let (shift_amount, mask) = get_shift_and_mask_for_flips(dir);
        let mut tmp_flips = 0;
        let mut tmp = shift_bits(move_bit, shift_amount) & mask;
        while (tmp != 0) && ((tmp & opponent_bits) != 0) {
            tmp_flips |= tmp;
            tmp = shift_bits(tmp, shift_amount) & mask;
        }
        if (tmp & player_bits) != 0 {
            flips |= tmp_flips;
        }
    }

    flips
}

impl BitBoard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init_board() -> Self {
        let mut board = Self::default();
        board.init();
        board
    }

    pub fn from_board(board: &(dyn Board + Send)) -> Self {
        let mut bit_board = Self::new();
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                let pos = Position::new(x, y);
                let color = board.get_cell_state(&pos);
                bit_board.set_cell_state(&pos, color);
            }
        }

        bit_board
    }
}

impl Board for BitBoard {
    fn cell_states(&self) -> [CellState; BOARD_SIZE * BOARD_SIZE] {
        let mut cells: [CellState; BOARD_SIZE * BOARD_SIZE] =
            [CellState::Empty; BOARD_SIZE * BOARD_SIZE];

        for y in 0..8 {
            for x in 0..8 {
                let index = y * BOARD_SIZE + x;
                cells[index] = self.get_cell_state(&Position::from_index(index));
            }
        }

        cells
    }

    fn get_cell_state(&self, pos: &Position) -> CellState {
        let bit = 1u64 << pos.to_index();
        if self.black & bit != 0 {
            CellState::Disc(Color::Black)
        } else if self.white & bit != 0 {
            CellState::Disc(Color::White)
        } else {
            CellState::Empty
        }
    }

    fn set_cell_state(&mut self, pos: &Position, cell: CellState) {
        let bit = 1u64 << pos.to_index();
        match cell {
            CellState::Disc(Color::Black) => {
                self.black |= bit;
                self.white &= !bit;
            }
            CellState::Disc(Color::White) => {
                self.white |= bit;
                self.black &= !bit;
            }
            CellState::Empty => {
                self.black &= !bit;
                self.white &= !bit;
            }
        }
    }

    fn count_of(&self, cell_state: CellState) -> usize {
        match cell_state {
            CellState::Disc(Color::Black) => self.black.count_ones() as usize,
            CellState::Disc(Color::White) => self.white.count_ones() as usize,
            CellState::Empty => (64 - self.black.count_ones() - self.white.count_ones()) as usize,
        }
    }

    fn make_move(&mut self, color: Color, pos: &Position) -> bool {
        let move_bit = 1u64 << pos.to_index();

        let (player_bits, opponent_bits) = match color {
            Color::Black => (&mut self.black, &mut self.white),
            Color::White => (&mut self.white, &mut self.black),
        };
        let valid_moves = get_valid_moves_bits(*player_bits, *opponent_bits);

        if valid_moves & move_bit == 0 {
            // Invalid move
            return false;
        }

        let flips = get_flips_bits(move_bit, *player_bits, *opponent_bits);

        *player_bits |= move_bit | flips;
        *opponent_bits &= !flips;

        true
    }

    fn get_valid_moves(&self, color: Color) -> Vec<Position> {
        let (player_bits, opponent_bits) = match color {
            Color::Black => (self.black, self.white),
            Color::White => (self.white, self.black),
        };
        let mut bits = get_valid_moves_bits(player_bits, opponent_bits);
        let mut moves = Vec::new();
        while bits != 0 {
            let lsb = bits & (!bits + 1); // 最下位の1ビットを取得
            let index = lsb.trailing_zeros() as usize;
            moves.push(Position::from_index(index));
            bits &= bits - 1; // 最下位の1ビットをクリア
        }
        moves
    }

    fn display(&self) {
        println!("  A B C D E F G H");
        for y in 0..BOARD_SIZE as u8 {
            print!("{}", y + 1);
            for x in 0..BOARD_SIZE as u8 {
                print!(" ");
                match self.get_cell_state(&Position { x, y }) {
                    CellState::Disc(Color::Black) => print!("B"), // 黒の駒
                    CellState::Disc(Color::White) => print!("W"), // 白の駒
                    CellState::Empty => print!("-"),              // 駒なし
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{Color, Position};

    #[test]
    fn test_get_valid_moves_all_directions() {
        // 全ての方向での合法手をテストするためのボード設定
        let mut board = BitBoard::default();

        // 中央に黒の駒を配置
        board.set_cell_state(&Position { x: 3, y: 3 }, CellState::Disc(Color::Black));

        // 黒の駒の周囲に白の駒を配置
        board.set_cell_state(&Position { x: 2, y: 2 }, CellState::Disc(Color::White)); // 北西
        board.set_cell_state(&Position { x: 3, y: 2 }, CellState::Disc(Color::White)); // 北
        board.set_cell_state(&Position { x: 4, y: 2 }, CellState::Disc(Color::White)); // 北東
        board.set_cell_state(&Position { x: 2, y: 3 }, CellState::Disc(Color::White)); // 西
        board.set_cell_state(&Position { x: 4, y: 3 }, CellState::Disc(Color::White)); // 東
        board.set_cell_state(&Position { x: 2, y: 4 }, CellState::Disc(Color::White)); // 南西
        board.set_cell_state(&Position { x: 3, y: 4 }, CellState::Disc(Color::White)); // 南
        board.set_cell_state(&Position { x: 4, y: 4 }, CellState::Disc(Color::White)); // 南東

        // 黒の合法手を取得
        let valid_moves_black = board.get_valid_moves(Color::Black);

        // 期待される合法手（白の駒のさらに先の位置）
        let expected_moves_black = vec![
            Position { x: 1, y: 1 }, // 北西
            Position { x: 3, y: 1 }, // 北
            Position { x: 5, y: 1 }, // 北東
            Position { x: 1, y: 3 }, // 西
            Position { x: 5, y: 3 }, // 東
            Position { x: 1, y: 5 }, // 南西
            Position { x: 3, y: 5 }, // 南
            Position { x: 5, y: 5 }, // 南東
        ];

        // ソートして比較
        let mut valid_moves_black_sorted = valid_moves_black.clone();
        valid_moves_black_sorted.sort_by_key(|p| (p.y, p.x));

        let mut expected_moves_black_sorted = expected_moves_black.clone();
        expected_moves_black_sorted.sort_by_key(|p| (p.y, p.x));

        assert_eq!(valid_moves_black_sorted, expected_moves_black_sorted);
    }

    #[test]
    fn test_get_valid_moves_edges_and_corners() {
        // ボードの端と隅での合法手をテスト

        // 上端でのテスト
        let mut board = BitBoard::default();
        board.set_cell_state(&Position { x: 0, y: 0 }, CellState::Disc(Color::Black)); // 左上隅
        board.set_cell_state(&Position { x: 1, y: 0 }, CellState::Disc(Color::White)); // 東
        board.set_cell_state(&Position { x: 0, y: 1 }, CellState::Disc(Color::White)); // 南
        board.set_cell_state(&Position { x: 1, y: 1 }, CellState::Disc(Color::White)); // 南東

        // 黒の合法手を取得
        let valid_moves_black = board.get_valid_moves(Color::Black);

        // 期待される合法手
        let expected_moves_black = vec![
            Position { x: 2, y: 0 }, // 東方向への合法手
            Position { x: 0, y: 2 }, // 南方向への合法手
            Position { x: 2, y: 2 }, // 南東方向への合法手
        ];

        // ソートして比較
        let mut valid_moves_black_sorted = valid_moves_black.clone();
        valid_moves_black_sorted.sort_by_key(|p| (p.y, p.x));

        let mut expected_moves_black_sorted = expected_moves_black.clone();
        expected_moves_black_sorted.sort_by_key(|p| (p.y, p.x));

        assert_eq!(valid_moves_black_sorted, expected_moves_black_sorted);

        // 下端でのテスト
        let mut board = BitBoard::default();
        board.set_cell_state(&Position { x: 7, y: 7 }, CellState::Disc(Color::Black)); // 右下隅
        board.set_cell_state(&Position { x: 6, y: 7 }, CellState::Disc(Color::White)); // 西
        board.set_cell_state(&Position { x: 7, y: 6 }, CellState::Disc(Color::White)); // 北
        board.set_cell_state(&Position { x: 6, y: 6 }, CellState::Disc(Color::White)); // 北西

        // 黒の合法手を取得
        let valid_moves_black = board.get_valid_moves(Color::Black);

        // 期待される合法手
        let expected_moves_black = vec![
            Position { x: 5, y: 7 }, // 西方向への合法手
            Position { x: 7, y: 5 }, // 北方向への合法手
            Position { x: 5, y: 5 }, // 北西方向への合法手
        ];

        // ソートして比較
        let mut valid_moves_black_sorted = valid_moves_black.clone();
        valid_moves_black_sorted.sort_by_key(|p| (p.y, p.x));

        let mut expected_moves_black_sorted = expected_moves_black.clone();
        expected_moves_black_sorted.sort_by_key(|p| (p.y, p.x));

        assert_eq!(valid_moves_black_sorted, expected_moves_black_sorted);
    }

    // 以前のテストも含めます
    #[test]
    fn test_get_valid_moves_initial_position() {
        // 初期のオセロボードを設定
        let mut board = BitBoard::default();

        // 初期配置
        board.set_cell_state(&Position { x: 3, y: 3 }, CellState::Disc(Color::White));
        board.set_cell_state(&Position { x: 4, y: 4 }, CellState::Disc(Color::White));
        board.set_cell_state(&Position { x: 3, y: 4 }, CellState::Disc(Color::Black));
        board.set_cell_state(&Position { x: 4, y: 3 }, CellState::Disc(Color::Black));

        // 黒の合法手を取得
        let valid_moves_black = board.get_valid_moves(Color::Black);

        // 期待される合法手
        let expected_moves_black = vec![
            Position { x: 2, y: 3 },
            Position { x: 3, y: 2 },
            Position { x: 4, y: 5 },
            Position { x: 5, y: 4 },
        ];

        // ソートして比較
        let mut valid_moves_black_sorted = valid_moves_black.clone();
        valid_moves_black_sorted.sort_by_key(|p| (p.y, p.x));

        let mut expected_moves_black_sorted = expected_moves_black.clone();
        expected_moves_black_sorted.sort_by_key(|p| (p.y, p.x));

        assert_eq!(valid_moves_black_sorted, expected_moves_black_sorted);

        // 白の合法手を取得
        let valid_moves_white = board.get_valid_moves(Color::White);

        // 期待される合法手
        let expected_moves_white = vec![
            Position { x: 2, y: 4 },
            Position { x: 3, y: 5 },
            Position { x: 4, y: 2 },
            Position { x: 5, y: 3 },
        ];

        // ソートして比較
        let mut valid_moves_white_sorted = valid_moves_white.clone();
        valid_moves_white_sorted.sort_by_key(|p| (p.y, p.x));

        let mut expected_moves_white_sorted = expected_moves_white.clone();
        expected_moves_white_sorted.sort_by_key(|p| (p.y, p.x));

        assert_eq!(valid_moves_white_sorted, expected_moves_white_sorted);
    }

    #[test]
    fn test_get_valid_moves_custom_position() {
        let mut board = BitBoard::default();

        board.set_cell_state(&Position::A1, CellState::Disc(Color::White));
        board.set_cell_state(&Position::A2, CellState::Disc(Color::Black));

        board.display();

        let valid_moves_white = board.get_valid_moves(Color::White);

        let mut tmp_board = BitBoard::default();
        valid_moves_white
            .iter()
            .for_each(|p| tmp_board.set_cell_state(p, CellState::Disc(Color::White)));
        tmp_board.display();

        let expected_moves_white = vec![Position::A3];

        // ソートして比較
        let mut valid_moves_white_sorted = valid_moves_white.clone();
        valid_moves_white_sorted.sort_by_key(|p| (p.y, p.x));

        let mut expected_moves_white_sorted = expected_moves_white.clone();
        expected_moves_white_sorted.sort_by_key(|p| (p.y, p.x));

        assert_eq!(valid_moves_white_sorted, expected_moves_white_sorted);
    }

    //   A B C D E F G H
    // 1 - - - - - - - -
    // 2 - - - - - - - -
    // 3 - - - - - - W B
    // 4 - - - - - - - -
    // 5 - - - - - - - -
    // 6 - - - - - - - -
    // 7 - - - - - - - -
    // 8 - - - - - - - -
    #[test]
    fn test_make_move() {
        let mut board = BitBoard::default();

        board.set_cell_state(&Position::A1, CellState::Disc(Color::Black));
        board.set_cell_state(&Position::A2, CellState::Disc(Color::White));

        let moves = board.get_valid_moves(Color::Black);

        board.make_move(Color::Black, &moves[0]);

        let color = board.get_cell_state(&Position::A2);
        board.display();

        assert_eq!(color, CellState::Disc(Color::Black));
    }
}
