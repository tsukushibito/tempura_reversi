const A1: u8 = 0;
const B1: u8 = 1;
const C1: u8 = 2;
const D1: u8 = 3;
const E1: u8 = 4;
const F1: u8 = 5;
const G1: u8 = 6;
const H1: u8 = 7;

const A2: u8 = 8;
const B2: u8 = 9;
const C2: u8 = 10;
const D2: u8 = 11;
const E2: u8 = 12;
const F2: u8 = 13;
const G2: u8 = 14;
const H2: u8 = 15;

const A3: u8 = 16;
const B3: u8 = 17;
const C3: u8 = 18;
const D3: u8 = 19;
const E3: u8 = 20;
const F3: u8 = 21;
const G3: u8 = 22;
const H3: u8 = 23;

const A4: u8 = 24;
const B4: u8 = 25;
const C4: u8 = 26;
const D4: u8 = 27;
const E4: u8 = 28;
const F4: u8 = 29;
const G4: u8 = 30;
const H4: u8 = 31;

const A5: u8 = 32;
const B5: u8 = 33;
const C5: u8 = 34;
const D5: u8 = 35;
const E5: u8 = 36;
const F5: u8 = 37;
const G5: u8 = 38;
const H5: u8 = 39;

const A6: u8 = 40;
const B6: u8 = 41;
const C6: u8 = 42;
const D6: u8 = 43;
const E6: u8 = 44;
const F6: u8 = 45;
const G6: u8 = 46;
const H6: u8 = 47;

const A7: u8 = 48;
const B7: u8 = 49;
const C7: u8 = 50;
const D7: u8 = 51;
const E7: u8 = 52;
const F7: u8 = 53;
const G7: u8 = 54;
const H7: u8 = 55;

const A8: u8 = 56;
const B8: u8 = 57;
const C8: u8 = 58;
const D8: u8 = 59;
const E8: u8 = 60;
const F8: u8 = 61;
const G8: u8 = 62;
const H8: u8 = 63;

const fn u8_to_coordinate(num: u8) -> (u8, u8) {
    let x = num % 8;
    let y = num / 8;
    (x, y)
}

const fn coordinate_to_u8(coord: (u8, u8)) -> u8 {
    coord.0 + coord.1 * 8
}

const fn rotate_90_cw_coord(coord: (u8, u8)) -> (u8, u8) {
    (7 - coord.1, coord.0)
}

const fn rotate_90_cw_u8(num: u8) -> u8 {
    let coord = u8_to_coordinate(num);
    let coord = rotate_90_cw_coord(coord);
    coordinate_to_u8(coord)
}

const fn rotate_90_cw_pattern<const N: usize>(pattern: &[u8; N]) -> [u8; N] {
    let mut rotated: [u8; N] = [0; N];

    let mut i = 0;
    while i < N {
        rotated[i] = rotate_90_cw_u8(pattern[i]);
        i += 1;
    }

    rotated
}

// PATTERN_00_x: 2nd row line feature (cells A2–H2)
// - - - - - - - -
// ● ● ● ● ● ● ● ●
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
pub const PATTERN_00_0: [u8; 8] = [A2, B2, C2, D2, E2, F2, G2, H2];
pub const PATTERN_00_1: [u8; 8] = rotate_90_cw_pattern(&PATTERN_00_0);
pub const PATTERN_00_2: [u8; 8] = rotate_90_cw_pattern(&PATTERN_00_1);
pub const PATTERN_00_3: [u8; 8] = rotate_90_cw_pattern(&PATTERN_00_2);

// PATTERN_01_x: 3rd row line feature (cells A3–H3)
// Visual:
// - - - - - - - -
// - - - - - - - -
// ● ● ● ● ● ● ● ●
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
pub const PATTERN_01_0: [u8; 8] = [A3, B3, C3, D3, E3, F3, G3, H3];
pub const PATTERN_01_1: [u8; 8] = rotate_90_cw_pattern(&PATTERN_01_0);
pub const PATTERN_01_2: [u8; 8] = rotate_90_cw_pattern(&PATTERN_01_1);
pub const PATTERN_01_3: [u8; 8] = rotate_90_cw_pattern(&PATTERN_01_2);

// PATTERN_02_x: 4th row line feature (cells A4–H4)
// Visual:
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
// ● ● ● ● ● ● ● ●
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
pub const PATTERN_02_0: [u8; 8] = [A4, B4, C4, D4, E4, F4, G4, H4];
pub const PATTERN_02_1: [u8; 8] = rotate_90_cw_pattern(&PATTERN_02_0);
pub const PATTERN_02_2: [u8; 8] = rotate_90_cw_pattern(&PATTERN_02_1);
pub const PATTERN_02_3: [u8; 8] = rotate_90_cw_pattern(&PATTERN_02_2);

// PATTERN_03_x: "Diagonal" feature (diagonal line from A1 to H8)
// Visual:
// ● ● - - - - - -
// ● ● - - - - - -
// - - ● - - - - -
// - - - ● - - - -
// - - - - ● - - -
// - - - - - ● - -
// - - - - - - ● -
// - - - - - - - ●
pub const PATTERN_03_0: [u8; 10] = [A1, B2, C3, D4, E5, F6, G7, H8, B1, A2];
pub const PATTERN_03_1: [u8; 10] = rotate_90_cw_pattern(&PATTERN_03_0);
pub const PATTERN_03_2: [u8; 10] = rotate_90_cw_pattern(&PATTERN_03_1);
pub const PATTERN_03_3: [u8; 10] = rotate_90_cw_pattern(&PATTERN_03_2);

// PATTERN_04_x: "Diagonal" feature (diagonal line from B1 to H7)
// Visual:
// - ● - - - - - -
// - - ● - - - - -
// - - - ● - - - -
// - - - - ● - - -
// - - - - - ● - -
// - - - - - - ● -
// - - - - - - - ●
// - - - - - - - -
pub const PATTERN_04_0: [u8; 7] = [B1, C2, D3, E4, F5, G6, H7];
pub const PATTERN_04_1: [u8; 7] = rotate_90_cw_pattern(&PATTERN_04_0);
pub const PATTERN_04_2: [u8; 7] = rotate_90_cw_pattern(&PATTERN_04_1);
pub const PATTERN_04_3: [u8; 7] = rotate_90_cw_pattern(&PATTERN_04_2);

// PATTERN_05_x: "Diagonal" feature (diagonal line from C1 to H6)
// Visual:
// - - ● - - - - -
// - - - ● - - - -
// - - - - ● - - -
// - - - - - ● - -
// - - - - - - ● -
// - - - - - - - ●
// - - - - - - - -
// - - - - - - - -
pub const PATTERN_05_0: [u8; 6] = [C1, D2, E3, F4, G5, H6];
pub const PATTERN_05_1: [u8; 6] = rotate_90_cw_pattern(&PATTERN_05_0);
pub const PATTERN_05_2: [u8; 6] = rotate_90_cw_pattern(&PATTERN_05_1);
pub const PATTERN_05_3: [u8; 6] = rotate_90_cw_pattern(&PATTERN_05_2);

// PATTERN_06_x: "Diagonal" feature (diagonal line from D1 to H5)
// Visual:
// - - - ● - - - -
// - - - - ● - - -
// - - - - - ● - -
// - - - - - - ● -
// - - - - - - - ●
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
pub const PATTERN_06_0: [u8; 5] = [D1, E2, F3, G4, H5];
pub const PATTERN_06_1: [u8; 5] = rotate_90_cw_pattern(&PATTERN_06_0);
pub const PATTERN_06_2: [u8; 5] = rotate_90_cw_pattern(&PATTERN_06_1);
pub const PATTERN_06_3: [u8; 5] = rotate_90_cw_pattern(&PATTERN_06_2);

// PATTERN_07_x: "Edge and X" feature (top row with additional X influence)
// Visual:
// ● ● ● ● ● ● ● ●
// - ● - - - - ● -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
pub const PATTERN_07_0: [u8; 10] = [A1, B1, C1, D1, E1, F1, G1, H1, B2, G2];
pub const PATTERN_07_1: [u8; 10] = rotate_90_cw_pattern(&PATTERN_07_0);
pub const PATTERN_07_2: [u8; 10] = rotate_90_cw_pattern(&PATTERN_07_1);
pub const PATTERN_07_3: [u8; 10] = rotate_90_cw_pattern(&PATTERN_07_2);

// PATTERN_08_x: "Edge" feature (top row: A1–H1 with additional C2, F2)
// Visual:
// ● ● ● ● ● ● ● ●
// - - ● - - ● - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
pub const PATTERN_08_0: [u8; 10] = [A1, B1, C1, D1, E1, F1, G1, H1, C2, F2];
pub const PATTERN_08_1: [u8; 10] = rotate_90_cw_pattern(&PATTERN_08_0);
pub const PATTERN_08_2: [u8; 10] = rotate_90_cw_pattern(&PATTERN_08_1);
pub const PATTERN_08_3: [u8; 10] = rotate_90_cw_pattern(&PATTERN_08_2);

// PATTERN_09_x: "Edge" feature (top block C1-F2 with corner A1, H1)
// Visual:
// ● - ● ● ● ● - ●
// - - ● ● ● ● - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
pub const PATTERN_09_0: [u8; 10] = [C1, D1, E1, F1, C2, D2, E2, F2, A1, H1];
pub const PATTERN_09_1: [u8; 10] = rotate_90_cw_pattern(&PATTERN_09_0);
pub const PATTERN_09_2: [u8; 10] = rotate_90_cw_pattern(&PATTERN_09_1);
pub const PATTERN_09_3: [u8; 10] = rotate_90_cw_pattern(&PATTERN_09_2);

// PATTERN_10_x: "Edge" feature (top block)
// Visual:
// - - ● ● ● ● - -
// - - - ● ● - - -
// - - ● ● ● ● - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
pub const PATTERN_10_0: [u8; 10] = [C1, D1, E1, F1, D2, E2, C3, D3, E3, F3];
pub const PATTERN_10_1: [u8; 10] = rotate_90_cw_pattern(&PATTERN_10_0);
pub const PATTERN_10_2: [u8; 10] = rotate_90_cw_pattern(&PATTERN_10_1);
pub const PATTERN_10_3: [u8; 10] = rotate_90_cw_pattern(&PATTERN_10_2);

// PATTERN_11_x: "Corner" feature (top left corner: A1–C3)
// Visual:
// ● ● ● - - - - -
// ● ● ● - - - - -
// ● ● ● - - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
pub const PATTERN_11_0: [u8; 9] = [A1, B1, C1, A2, B2, C2, A3, B3, C3];
pub const PATTERN_11_1: [u8; 9] = rotate_90_cw_pattern(&PATTERN_11_0);
pub const PATTERN_11_2: [u8; 9] = rotate_90_cw_pattern(&PATTERN_11_1);
pub const PATTERN_11_3: [u8; 9] = rotate_90_cw_pattern(&PATTERN_11_2);

// PATTERN_12_x: "Corner" feature (top left corner, triangular shape)
// Visual:
// ● ● ● ● - - - -
// ● ● ● - - - - -
// ● ● - - - - - -
// ● - - - - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
pub const PATTERN_12_0: [u8; 10] = [A1, B1, C1, D1, A2, B2, C2, A3, B3, A4];
pub const PATTERN_12_1: [u8; 10] = rotate_90_cw_pattern(&PATTERN_12_0);
pub const PATTERN_12_2: [u8; 10] = rotate_90_cw_pattern(&PATTERN_12_1);
pub const PATTERN_12_3: [u8; 10] = rotate_90_cw_pattern(&PATTERN_12_2);

// PATTERN_13_x: "Corner" feature (top left corner, triangular shape 2)
// Visual:
// ● ● ● ● ● - - -
// ● ● - - - - - -
// ● - - - - - - -
// ● - - - - - - -
// ● - - - - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
pub const PATTERN_13_0: [u8; 10] = [A1, B1, C1, D1, E1, A2, B2, A3, A4, A5];
pub const PATTERN_13_1: [u8; 10] = rotate_90_cw_pattern(&PATTERN_13_0);
pub const PATTERN_13_2: [u8; 10] = rotate_90_cw_pattern(&PATTERN_13_1);
pub const PATTERN_13_3: [u8; 10] = rotate_90_cw_pattern(&PATTERN_13_2);

// PATTERN_14_x: "Corner" feature (top left corner, diagonal)
// Visual:
// ● ● - - - - - -
// ● ● ● - - - - -
// - ● ● ● - - - -
// - - ● ● - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
pub const PATTERN_14_0: [u8; 10] = [A1, B1, A2, B2, C2, B3, C3, D3, C4, D4];
pub const PATTERN_14_1: [u8; 10] = rotate_90_cw_pattern(&PATTERN_14_0);
pub const PATTERN_14_2: [u8; 10] = rotate_90_cw_pattern(&PATTERN_14_1);
pub const PATTERN_14_3: [u8; 10] = rotate_90_cw_pattern(&PATTERN_14_2);

// PATTERN_15_x: "Corner" feature (top left corner, diagonal 2)
// Visual:
// ● ● - - - - - -
// ● ● ● ● - - - -
// - ● ● - - - - -
// - ● - ● - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
pub const PATTERN_15_0: [u8; 10] = [A1, B1, A2, B2, C2, D2, B3, C3, B4, D4];
pub const PATTERN_15_1: [u8; 10] = rotate_90_cw_pattern(&PATTERN_15_0);
pub const PATTERN_15_2: [u8; 10] = rotate_90_cw_pattern(&PATTERN_15_1);
pub const PATTERN_15_3: [u8; 10] = rotate_90_cw_pattern(&PATTERN_15_2);

pub struct Pattern {
    pub pattern: &'static [u8],
}

pub const PATTERNS: [&[u8]; 16 * 4] = [
    &PATTERN_00_0,
    &PATTERN_00_1,
    &PATTERN_00_2,
    &PATTERN_00_3,
    &PATTERN_01_0,
    &PATTERN_01_1,
    &PATTERN_01_2,
    &PATTERN_01_3,
    &PATTERN_02_0,
    &PATTERN_02_1,
    &PATTERN_02_2,
    &PATTERN_02_3,
    &PATTERN_03_0,
    &PATTERN_03_1,
    &PATTERN_03_2,
    &PATTERN_03_3,
    &PATTERN_04_0,
    &PATTERN_04_1,
    &PATTERN_04_2,
    &PATTERN_04_3,
    &PATTERN_05_0,
    &PATTERN_05_1,
    &PATTERN_05_2,
    &PATTERN_05_3,
    &PATTERN_06_0,
    &PATTERN_06_1,
    &PATTERN_06_2,
    &PATTERN_06_3,
    &PATTERN_07_0,
    &PATTERN_07_1,
    &PATTERN_07_2,
    &PATTERN_07_3,
    &PATTERN_08_0,
    &PATTERN_08_1,
    &PATTERN_08_2,
    &PATTERN_08_3,
    &PATTERN_09_0,
    &PATTERN_09_1,
    &PATTERN_09_2,
    &PATTERN_09_3,
    &PATTERN_10_0,
    &PATTERN_10_1,
    &PATTERN_10_2,
    &PATTERN_10_3,
    &PATTERN_11_0,
    &PATTERN_11_1,
    &PATTERN_11_2,
    &PATTERN_11_3,
    &PATTERN_12_0,
    &PATTERN_12_1,
    &PATTERN_12_2,
    &PATTERN_12_3,
    &PATTERN_13_0,
    &PATTERN_13_1,
    &PATTERN_13_2,
    &PATTERN_13_3,
    &PATTERN_14_0,
    &PATTERN_14_1,
    &PATTERN_14_2,
    &PATTERN_14_3,
    &PATTERN_15_0,
    &PATTERN_15_1,
    &PATTERN_15_2,
    &PATTERN_15_3,
];

const fn c2f_count(coord: u8) -> usize {
    let mut pattern_index = 0;
    let mut count = 0;
    while pattern_index < PATTERNS.len() {
        let mut feature_index = 0;
        while feature_index < PATTERNS[pattern_index].len() {
            if PATTERNS[pattern_index][feature_index] == coord {
                count += 1;
            }
            feature_index += 1;
        }
        pattern_index += 1;
    }
    count
}

#[derive(Debug, Clone, Copy)]
pub struct CoordinateToFeature {
    pub pattern_index: u32,
    pub feature_index: u32,
}

pub const fn make_c2f_list<const TARGET: u8, const COUNT: usize>() -> [CoordinateToFeature; COUNT] {
    let mut list: [CoordinateToFeature; COUNT] = [CoordinateToFeature {
        pattern_index: 0,
        feature_index: 0,
    }; COUNT];
    let mut pattern_index = 0;
    let mut list_index = 0;
    while pattern_index < PATTERNS.len() {
        let mut feature_index = 0;
        while feature_index < PATTERNS[pattern_index].len() {
            if PATTERNS[pattern_index][feature_index] == TARGET {
                let feature_index = 3u32.pow(feature_index as u32);
                list[list_index] = CoordinateToFeature {
                    pattern_index: pattern_index as u32,
                    feature_index: feature_index,
                };
                list_index += 1;
            }
            feature_index += 1;
        }
        pattern_index += 1;
    }
    list
}

pub const A1_C2F_COUNT: usize = c2f_count(A1);
pub const A1_C2F_LIST: [CoordinateToFeature; A1_C2F_COUNT] = make_c2f_list::<A1, A1_C2F_COUNT>();
pub const A2_C2F_COUNT: usize = c2f_count(A2);
pub const A2_C2F_LIST: [CoordinateToFeature; A2_C2F_COUNT] = make_c2f_list::<A2, A2_C2F_COUNT>();
pub const A3_C2F_COUNT: usize = c2f_count(A3);
pub const A3_C2F_LIST: [CoordinateToFeature; A3_C2F_COUNT] = make_c2f_list::<A3, A3_C2F_COUNT>();

// pub const COORDINATE_TO_FEATURE_INDEX: [u8; 64] = [];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_90_cw_pattern() {
        let pattern: [u8; 4] = [A1, B1, A2, B2];
        let rotated = rotate_90_cw_pattern(&pattern);

        assert_eq!(rotated[0], H1);
        assert_eq!(rotated[1], H2);
        assert_eq!(rotated[2], G1);
        assert_eq!(rotated[3], G2);
    }

    #[test]
    fn test_make_c2f_list() {
        let list = make_c2f_list::<A1, A1_C2F_COUNT>();
        assert_eq!(list.len(), A1_C2F_COUNT);
        assert_eq!(list[0].pattern_index, 12);
        assert_eq!(list[0].feature_index, 1);
    }
}
