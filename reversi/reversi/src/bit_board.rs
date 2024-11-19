use crate::{
    board::{Board, BOARD_SIZE},
    Color, Direction, Position,
};

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq)]
pub struct BitBoard {
    black: u64,
    white: u64,
}

fn get_shift_and_mask(dir: Direction) -> (i32, u64) {
    const MASK_EXCLUDE_A_FILE: u64 = 0xfefefefefefefefeu64;
    const MASK_EXCLUDE_H_FILE: u64 = 0x7f7f7f7f7f7f7f7fu64;
    const MASK_EXCLUDE_FIRST_RANK: u64 = 0xffffffffffffff00u64;
    const MASK_EXCLUDE_LAST_RANK: u64 = 0x00ffffffffffffffu64;
    match dir {
        Direction::East => (1, MASK_EXCLUDE_A_FILE),
        Direction::West => (-1, MASK_EXCLUDE_H_FILE),
        Direction::South => (8, MASK_EXCLUDE_FIRST_RANK),
        Direction::North => (-8, MASK_EXCLUDE_LAST_RANK),
        Direction::SouthEast => (
            9,
            MASK_EXCLUDE_FIRST_RANK & MASK_EXCLUDE_A_FILE & MASK_EXCLUDE_H_FILE,
        ),
        Direction::SouthWest => (
            7,
            MASK_EXCLUDE_FIRST_RANK & MASK_EXCLUDE_A_FILE & MASK_EXCLUDE_H_FILE,
        ),
        Direction::NorthEast => (
            -7,
            MASK_EXCLUDE_LAST_RANK & MASK_EXCLUDE_A_FILE & MASK_EXCLUDE_H_FILE,
        ),
        Direction::NorthWest => (
            -9,
            MASK_EXCLUDE_LAST_RANK & MASK_EXCLUDE_A_FILE & MASK_EXCLUDE_H_FILE,
        ),
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
        let (shift_amount, mask) = get_shift_and_mask(dir);
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
        let (shift_amount, mask) = get_shift_and_mask(dir);
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

impl Board for BitBoard {
    fn discs(&self) -> Vec<Vec<Option<Color>>> {
        let mut discs: Vec<Vec<Option<Color>>> = Vec::new();

        for y in 0..8 {
            let mut row: Vec<Option<Color>> = Vec::new();
            for x in 0..8 {
                row.push(self.get_disc(&Position { x, y }));
            }
            discs.push(row);
        }

        discs
    }

    fn get_disc(&self, pos: &Position) -> Option<Color> {
        let index = pos.y * BOARD_SIZE as i8 + pos.x;
        let bit = 1u64 << index;
        if self.black & bit != 0 {
            Some(Color::Black)
        } else if self.white & bit != 0 {
            Some(Color::White)
        } else {
            None
        }
    }

    fn set_disc(&mut self, pos: &Position, color: Option<Color>) {
        let index = pos.y * BOARD_SIZE as i8 + pos.x;
        let bit = 1u64 << index;
        match color {
            Some(Color::Black) => {
                self.black |= bit;
                self.white &= !bit;
            }
            Some(Color::White) => {
                self.white |= bit;
                self.black &= !bit;
            }
            None => {
                self.black &= !bit;
                self.white &= !bit;
            }
        }
    }

    fn count_of(&self, color: Option<Color>) -> usize {
        match color {
            Some(c) => match c {
                Color::Black => self.black.count_ones() as usize,
                Color::White => self.white.count_ones() as usize,
            },
            None => (64 - self.black.count_ones() - self.white.count_ones()) as usize,
        }
    }

    fn make_move(&mut self, color: Color, pos: &Position) -> bool {
        let idx = pos.x + pos.y * BOARD_SIZE as i8;
        let move_bit = 1u64 << idx;

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
            let x = (index % 8) as i8;
            let y = (index / 8) as i8;
            moves.push(Position { x, y });
            bits &= bits - 1; // 最下位の1ビットをクリア
        }
        moves
    }

    fn display(&self) {
        println!("  A B C D E F G H");
        for y in 0..BOARD_SIZE as i8 {
            print!("{}", y + 1);
            for x in 0..BOARD_SIZE as i8 {
                print!(" ");
                match self.get_disc(&Position { x, y }) {
                    Some(Color::Black) => print!("B"), // 黒の駒
                    Some(Color::White) => print!("W"), // 白の駒
                    None => print!("-"),               // 駒なし
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;
    use crate::{Color, Position};

    #[test]
    fn test_get_valid_moves_all_directions() {
        // 全ての方向での合法手をテストするためのボード設定
        let mut board = BitBoard::default();

        // 中央に黒の駒を配置
        board.set_disc(&Position { x: 3, y: 3 }, Some(Color::Black));

        // 黒の駒の周囲に白の駒を配置
        board.set_disc(&Position { x: 2, y: 2 }, Some(Color::White)); // 北西
        board.set_disc(&Position { x: 3, y: 2 }, Some(Color::White)); // 北
        board.set_disc(&Position { x: 4, y: 2 }, Some(Color::White)); // 北東
        board.set_disc(&Position { x: 2, y: 3 }, Some(Color::White)); // 西
        board.set_disc(&Position { x: 4, y: 3 }, Some(Color::White)); // 東
        board.set_disc(&Position { x: 2, y: 4 }, Some(Color::White)); // 南西
        board.set_disc(&Position { x: 3, y: 4 }, Some(Color::White)); // 南
        board.set_disc(&Position { x: 4, y: 4 }, Some(Color::White)); // 南東

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
        board.set_disc(&Position { x: 0, y: 0 }, Some(Color::Black)); // 左上隅
        board.set_disc(&Position { x: 1, y: 0 }, Some(Color::White)); // 東
        board.set_disc(&Position { x: 0, y: 1 }, Some(Color::White)); // 南
        board.set_disc(&Position { x: 1, y: 1 }, Some(Color::White)); // 南東

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
        board.set_disc(&Position { x: 7, y: 7 }, Some(Color::Black)); // 右下隅
        board.set_disc(&Position { x: 6, y: 7 }, Some(Color::White)); // 西
        board.set_disc(&Position { x: 7, y: 6 }, Some(Color::White)); // 北
        board.set_disc(&Position { x: 6, y: 6 }, Some(Color::White)); // 北西

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
        board.set_disc(&Position { x: 3, y: 3 }, Some(Color::White));
        board.set_disc(&Position { x: 4, y: 4 }, Some(Color::White));
        board.set_disc(&Position { x: 3, y: 4 }, Some(Color::Black));
        board.set_disc(&Position { x: 4, y: 3 }, Some(Color::Black));

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

        // 行1
        board.set_disc(&Position { x: 1, y: 0 }, Some(Color::Black));
        board.set_disc(&Position { x: 2, y: 0 }, Some(Color::Black));
        board.set_disc(&Position { x: 3, y: 0 }, Some(Color::Black));
        board.set_disc(&Position { x: 4, y: 0 }, Some(Color::Black));
        board.set_disc(&Position { x: 5, y: 0 }, Some(Color::Black));
        board.set_disc(&Position { x: 6, y: 0 }, Some(Color::Black));
        board.set_disc(&Position { x: 7, y: 0 }, Some(Color::Black));

        // 行2
        board.set_disc(&Position { x: 3, y: 1 }, Some(Color::White));
        board.set_disc(&Position { x: 4, y: 1 }, Some(Color::Black));
        board.set_disc(&Position { x: 6, y: 1 }, Some(Color::Black));

        // 行3
        board.set_disc(&Position { x: 2, y: 2 }, Some(Color::White));
        board.set_disc(&Position { x: 3, y: 2 }, Some(Color::Black));
        board.set_disc(&Position { x: 4, y: 2 }, Some(Color::White));
        board.set_disc(&Position { x: 5, y: 2 }, Some(Color::Black));
        board.set_disc(&Position { x: 6, y: 2 }, Some(Color::Black));
        board.set_disc(&Position { x: 7, y: 2 }, Some(Color::White));

        // 行4
        board.set_disc(&Position { x: 3, y: 3 }, Some(Color::Black));
        board.set_disc(&Position { x: 4, y: 3 }, Some(Color::White));
        board.set_disc(&Position { x: 5, y: 3 }, Some(Color::White));
        board.set_disc(&Position { x: 6, y: 3 }, Some(Color::White));
        board.set_disc(&Position { x: 7, y: 3 }, Some(Color::White));

        // 行5
        board.set_disc(&Position { x: 0, y: 4 }, Some(Color::White));
        board.set_disc(&Position { x: 1, y: 4 }, Some(Color::White));
        board.set_disc(&Position { x: 2, y: 4 }, Some(Color::White));
        board.set_disc(&Position { x: 3, y: 4 }, Some(Color::Black));
        board.set_disc(&Position { x: 4, y: 4 }, Some(Color::White));
        board.set_disc(&Position { x: 5, y: 4 }, Some(Color::White));
        board.set_disc(&Position { x: 6, y: 4 }, Some(Color::White));
        board.set_disc(&Position { x: 7, y: 4 }, Some(Color::White));

        // 行6
        board.set_disc(&Position { x: 2, y: 5 }, Some(Color::Black));
        board.set_disc(&Position { x: 3, y: 5 }, Some(Color::White));
        board.set_disc(&Position { x: 5, y: 5 }, Some(Color::White));
        board.set_disc(&Position { x: 6, y: 5 }, Some(Color::White));
        board.set_disc(&Position { x: 7, y: 5 }, Some(Color::White));

        // 行7
        board.set_disc(&Position { x: 2, y: 6 }, Some(Color::White));

        board.display();

        let valid_moves_white = board.get_valid_moves(Color::White);

        let mut tmp_board = BitBoard::default();
        valid_moves_white
            .iter()
            .for_each(|p| tmp_board.set_disc(p, Some(Color::White)));
        tmp_board.display();
        std::io::stdout().flush().unwrap();

        let expected_moves_white = vec![
            Position::C2,
            Position::F2,
            Position::H2,
            Position::C4,
            Position::B6,
            Position::B7,
            Position::D7,
        ];

        // ソートして比較
        let mut valid_moves_white_sorted = valid_moves_white.clone();
        valid_moves_white_sorted.sort_by_key(|p| (p.y, p.x));

        let mut expected_moves_white_sorted = expected_moves_white.clone();
        expected_moves_white_sorted.sort_by_key(|p| (p.y, p.x));

        assert_eq!(valid_moves_white_sorted, expected_moves_white_sorted);
    }

    //   A B C D E F G H
    // 1 B W W W W W - -
    // 2 W W W W W - - -
    #[test]
    fn test_make_move() {
        let mut board = BitBoard::default();

        // 行1
        board.set_disc(&Position { x: 0, y: 0 }, Some(Color::Black));
        board.set_disc(&Position { x: 1, y: 0 }, Some(Color::White));
        board.set_disc(&Position { x: 2, y: 0 }, Some(Color::White));
        board.set_disc(&Position { x: 3, y: 0 }, Some(Color::White));
        board.set_disc(&Position { x: 4, y: 0 }, Some(Color::White));
        board.set_disc(&Position { x: 5, y: 0 }, Some(Color::White));

        // 行2
        board.set_disc(&Position { x: 0, y: 1 }, Some(Color::White));
        board.set_disc(&Position { x: 1, y: 1 }, Some(Color::White));
        board.set_disc(&Position { x: 2, y: 1 }, Some(Color::White));
        board.set_disc(&Position { x: 3, y: 1 }, Some(Color::White));
        board.set_disc(&Position { x: 4, y: 1 }, Some(Color::White));

        board.make_move(Color::Black, &Position::G1);

        assert_eq!(board.get_disc(&Position::A1), Some(Color::Black));
        assert_eq!(board.get_disc(&Position::B1), Some(Color::Black));
        assert_eq!(board.get_disc(&Position::C1), Some(Color::Black));
        assert_eq!(board.get_disc(&Position::D1), Some(Color::Black));
        assert_eq!(board.get_disc(&Position::E1), Some(Color::Black));
        assert_eq!(board.get_disc(&Position::F1), Some(Color::Black));
        assert_eq!(board.get_disc(&Position::G1), Some(Color::Black));
    }
}
