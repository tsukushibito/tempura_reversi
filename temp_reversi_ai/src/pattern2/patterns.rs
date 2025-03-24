use temp_reversi_core::Position;

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

// PATTERN_0_x: 2nd row line feature (cells A2–H2)
pub const PATTERN_0_0: [u8; 8] = [A2, B2, C2, D2, E2, F2, G2, H2];
pub const PATTERN_0_1: [u8; 8] = rotate_90_cw_pattern(&PATTERN_0_0);
pub const PATTERN_0_2: [u8; 8] = rotate_90_cw_pattern(&PATTERN_0_1);
pub const PATTERN_0_3: [u8; 8] = rotate_90_cw_pattern(&PATTERN_0_2);

// PATTERN_1_x: 3rd row line feature (cells A3–H3)
pub const PATTERN_1_0: [u8; 8] = [A3, B3, C3, D3, E3, F3, G3, H3];
pub const PATTERN_1_1: [u8; 8] = rotate_90_cw_pattern(&PATTERN_1_0);
pub const PATTERN_1_2: [u8; 8] = rotate_90_cw_pattern(&PATTERN_1_1);
pub const PATTERN_1_3: [u8; 8] = rotate_90_cw_pattern(&PATTERN_1_2);

// PATTERN_2_x: 4th row line feature (cells A4–H4)
pub const PATTERN_2_0: [u8; 8] = [A4, B4, C4, D4, E4, F4, G4, H4];
pub const PATTERN_2_1: [u8; 8] = rotate_90_cw_pattern(&PATTERN_2_0);
pub const PATTERN_2_2: [u8; 8] = rotate_90_cw_pattern(&PATTERN_2_1);
pub const PATTERN_2_3: [u8; 8] = rotate_90_cw_pattern(&PATTERN_2_2);

// PATTERN_3_x: "Edge and X" feature (top row with additional X influence)
// This pattern covers the top edge (row 1: A1–H1) along with two extra cells (B2, G2)
pub const PATTERN_3_0: [u8; 10] = [A1, B1, C1, D1, E1, F1, G1, H1, B2, G2];
pub const PATTERN_3_1: [u8; 10] = rotate_90_cw_pattern(&PATTERN_3_0);
pub const PATTERN_3_2: [u8; 10] = rotate_90_cw_pattern(&PATTERN_3_1);
pub const PATTERN_3_3: [u8; 10] = rotate_90_cw_pattern(&PATTERN_3_2);

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
}
