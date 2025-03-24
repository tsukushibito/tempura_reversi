use super::coordinate::*;

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
