use crate::patterns::PATTERNS;

use super::coordinate::*;

/// Get the number of patterns that include the specified position.
pub const fn c2f_count(coord: u8) -> usize {
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

/// The `CoordToFeatureIndex` struct represents a mapping from a coordinate to a feature index.
/// It contains the pattern index and the trit place value for that coordinate.
#[derive(Debug, Clone, Copy)]
pub struct CoordToFeatureIndex {
    /// The index of the pattern in the `PATTERNS` array.
    /// This index is used to identify which pattern the coordinate belongs to.
    pub pattern_index: u8,

    /// The trit place value for the coordinate.
    /// This value is used to calculate the feature value for the coordinate.
    pub trit_place_value: u16,
}

/// Creates a list of `CoordToFeatureIndex` structs for a given target coordinate and count.
/// The list contains all the coordinates that map to the target coordinate in the `PATTERNS` array.
pub const fn make_c2f_list<const TARGET: u8, const COUNT: usize>() -> [CoordToFeatureIndex; COUNT] {
    let mut list: [CoordToFeatureIndex; COUNT] = [CoordToFeatureIndex {
        pattern_index: 0,
        trit_place_value: 0,
    }; COUNT];
    let mut pattern_index = 0;
    let mut list_index = 0;
    while pattern_index < PATTERNS.len() {
        let mut feature_index = 0;
        while feature_index < PATTERNS[pattern_index].len() {
            if PATTERNS[pattern_index][feature_index] == TARGET {
                let trit_place_value = 3u32.pow(feature_index as u32);
                list[list_index] = CoordToFeatureIndex {
                    pattern_index: pattern_index as u8,
                    trit_place_value: trit_place_value as u16,
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
pub const B1_C2F_COUNT: usize = c2f_count(B1);
pub const C1_C2F_COUNT: usize = c2f_count(C1);
pub const D1_C2F_COUNT: usize = c2f_count(D1);
pub const E1_C2F_COUNT: usize = c2f_count(E1);
pub const F1_C2F_COUNT: usize = c2f_count(F1);
pub const G1_C2F_COUNT: usize = c2f_count(G1);
pub const H1_C2F_COUNT: usize = c2f_count(H1);

pub const A2_C2F_COUNT: usize = c2f_count(A2);
pub const B2_C2F_COUNT: usize = c2f_count(B2);
pub const C2_C2F_COUNT: usize = c2f_count(C2);
pub const D2_C2F_COUNT: usize = c2f_count(D2);
pub const E2_C2F_COUNT: usize = c2f_count(E2);
pub const F2_C2F_COUNT: usize = c2f_count(F2);
pub const G2_C2F_COUNT: usize = c2f_count(G2);
pub const H2_C2F_COUNT: usize = c2f_count(H2);

pub const A3_C2F_COUNT: usize = c2f_count(A3);
pub const B3_C2F_COUNT: usize = c2f_count(B3);
pub const C3_C2F_COUNT: usize = c2f_count(C3);
pub const D3_C2F_COUNT: usize = c2f_count(D3);
pub const E3_C2F_COUNT: usize = c2f_count(E3);
pub const F3_C2F_COUNT: usize = c2f_count(F3);
pub const G3_C2F_COUNT: usize = c2f_count(G3);
pub const H3_C2F_COUNT: usize = c2f_count(H3);

pub const A4_C2F_COUNT: usize = c2f_count(A4);
pub const B4_C2F_COUNT: usize = c2f_count(B4);
pub const C4_C2F_COUNT: usize = c2f_count(C4);
pub const D4_C2F_COUNT: usize = c2f_count(D4);
pub const E4_C2F_COUNT: usize = c2f_count(E4);
pub const F4_C2F_COUNT: usize = c2f_count(F4);
pub const G4_C2F_COUNT: usize = c2f_count(G4);
pub const H4_C2F_COUNT: usize = c2f_count(H4);

pub const A5_C2F_COUNT: usize = c2f_count(A5);
pub const B5_C2F_COUNT: usize = c2f_count(B5);
pub const C5_C2F_COUNT: usize = c2f_count(C5);
pub const D5_C2F_COUNT: usize = c2f_count(D5);
pub const E5_C2F_COUNT: usize = c2f_count(E5);
pub const F5_C2F_COUNT: usize = c2f_count(F5);
pub const G5_C2F_COUNT: usize = c2f_count(G5);
pub const H5_C2F_COUNT: usize = c2f_count(H5);

pub const A6_C2F_COUNT: usize = c2f_count(A6);
pub const B6_C2F_COUNT: usize = c2f_count(B6);
pub const C6_C2F_COUNT: usize = c2f_count(C6);
pub const D6_C2F_COUNT: usize = c2f_count(D6);
pub const E6_C2F_COUNT: usize = c2f_count(E6);
pub const F6_C2F_COUNT: usize = c2f_count(F6);
pub const G6_C2F_COUNT: usize = c2f_count(G6);
pub const H6_C2F_COUNT: usize = c2f_count(H6);

pub const A7_C2F_COUNT: usize = c2f_count(A7);
pub const B7_C2F_COUNT: usize = c2f_count(B7);
pub const C7_C2F_COUNT: usize = c2f_count(C7);
pub const D7_C2F_COUNT: usize = c2f_count(D7);
pub const E7_C2F_COUNT: usize = c2f_count(E7);
pub const F7_C2F_COUNT: usize = c2f_count(F7);
pub const G7_C2F_COUNT: usize = c2f_count(G7);
pub const H7_C2F_COUNT: usize = c2f_count(H7);

pub const A8_C2F_COUNT: usize = c2f_count(A8);
pub const B8_C2F_COUNT: usize = c2f_count(B8);
pub const C8_C2F_COUNT: usize = c2f_count(C8);
pub const D8_C2F_COUNT: usize = c2f_count(D8);
pub const E8_C2F_COUNT: usize = c2f_count(E8);
pub const F8_C2F_COUNT: usize = c2f_count(F8);
pub const G8_C2F_COUNT: usize = c2f_count(G8);
pub const H8_C2F_COUNT: usize = c2f_count(H8);

pub const A1_C2F_LIST: [CoordToFeatureIndex; A1_C2F_COUNT] = make_c2f_list::<A1, A1_C2F_COUNT>();
pub const B1_C2F_LIST: [CoordToFeatureIndex; B1_C2F_COUNT] = make_c2f_list::<B1, B1_C2F_COUNT>();
pub const C1_C2F_LIST: [CoordToFeatureIndex; C1_C2F_COUNT] = make_c2f_list::<C1, C1_C2F_COUNT>();
pub const D1_C2F_LIST: [CoordToFeatureIndex; D1_C2F_COUNT] = make_c2f_list::<D1, D1_C2F_COUNT>();
pub const E1_C2F_LIST: [CoordToFeatureIndex; E1_C2F_COUNT] = make_c2f_list::<E1, E1_C2F_COUNT>();
pub const F1_C2F_LIST: [CoordToFeatureIndex; F1_C2F_COUNT] = make_c2f_list::<F1, F1_C2F_COUNT>();
pub const G1_C2F_LIST: [CoordToFeatureIndex; G1_C2F_COUNT] = make_c2f_list::<G1, G1_C2F_COUNT>();
pub const H1_C2F_LIST: [CoordToFeatureIndex; H1_C2F_COUNT] = make_c2f_list::<H1, H1_C2F_COUNT>();

pub const A2_C2F_LIST: [CoordToFeatureIndex; A2_C2F_COUNT] = make_c2f_list::<A2, A2_C2F_COUNT>();
pub const B2_C2F_LIST: [CoordToFeatureIndex; B2_C2F_COUNT] = make_c2f_list::<B2, B2_C2F_COUNT>();
pub const C2_C2F_LIST: [CoordToFeatureIndex; C2_C2F_COUNT] = make_c2f_list::<C2, C2_C2F_COUNT>();
pub const D2_C2F_LIST: [CoordToFeatureIndex; D2_C2F_COUNT] = make_c2f_list::<D2, D2_C2F_COUNT>();
pub const E2_C2F_LIST: [CoordToFeatureIndex; E2_C2F_COUNT] = make_c2f_list::<E2, E2_C2F_COUNT>();
pub const F2_C2F_LIST: [CoordToFeatureIndex; F2_C2F_COUNT] = make_c2f_list::<F2, F2_C2F_COUNT>();
pub const G2_C2F_LIST: [CoordToFeatureIndex; G2_C2F_COUNT] = make_c2f_list::<G2, G2_C2F_COUNT>();
pub const H2_C2F_LIST: [CoordToFeatureIndex; H2_C2F_COUNT] = make_c2f_list::<H2, H2_C2F_COUNT>();

pub const A3_C2F_LIST: [CoordToFeatureIndex; A3_C2F_COUNT] = make_c2f_list::<A3, A3_C2F_COUNT>();
pub const B3_C2F_LIST: [CoordToFeatureIndex; B3_C2F_COUNT] = make_c2f_list::<B3, B3_C2F_COUNT>();
pub const C3_C2F_LIST: [CoordToFeatureIndex; C3_C2F_COUNT] = make_c2f_list::<C3, C3_C2F_COUNT>();
pub const D3_C2F_LIST: [CoordToFeatureIndex; D3_C2F_COUNT] = make_c2f_list::<D3, D3_C2F_COUNT>();
pub const E3_C2F_LIST: [CoordToFeatureIndex; E3_C2F_COUNT] = make_c2f_list::<E3, E3_C2F_COUNT>();
pub const F3_C2F_LIST: [CoordToFeatureIndex; F3_C2F_COUNT] = make_c2f_list::<F3, F3_C2F_COUNT>();
pub const G3_C2F_LIST: [CoordToFeatureIndex; G3_C2F_COUNT] = make_c2f_list::<G3, G3_C2F_COUNT>();
pub const H3_C2F_LIST: [CoordToFeatureIndex; H3_C2F_COUNT] = make_c2f_list::<H3, H3_C2F_COUNT>();

pub const A4_C2F_LIST: [CoordToFeatureIndex; A4_C2F_COUNT] = make_c2f_list::<A4, A4_C2F_COUNT>();
pub const B4_C2F_LIST: [CoordToFeatureIndex; B4_C2F_COUNT] = make_c2f_list::<B4, B4_C2F_COUNT>();
pub const C4_C2F_LIST: [CoordToFeatureIndex; C4_C2F_COUNT] = make_c2f_list::<C4, C4_C2F_COUNT>();
pub const D4_C2F_LIST: [CoordToFeatureIndex; D4_C2F_COUNT] = make_c2f_list::<D4, D4_C2F_COUNT>();
pub const E4_C2F_LIST: [CoordToFeatureIndex; E4_C2F_COUNT] = make_c2f_list::<E4, E4_C2F_COUNT>();
pub const F4_C2F_LIST: [CoordToFeatureIndex; F4_C2F_COUNT] = make_c2f_list::<F4, F4_C2F_COUNT>();
pub const G4_C2F_LIST: [CoordToFeatureIndex; G4_C2F_COUNT] = make_c2f_list::<G4, G4_C2F_COUNT>();
pub const H4_C2F_LIST: [CoordToFeatureIndex; H4_C2F_COUNT] = make_c2f_list::<H4, H4_C2F_COUNT>();

pub const A5_C2F_LIST: [CoordToFeatureIndex; A5_C2F_COUNT] = make_c2f_list::<A5, A5_C2F_COUNT>();
pub const B5_C2F_LIST: [CoordToFeatureIndex; B5_C2F_COUNT] = make_c2f_list::<B5, B5_C2F_COUNT>();
pub const C5_C2F_LIST: [CoordToFeatureIndex; C5_C2F_COUNT] = make_c2f_list::<C5, C5_C2F_COUNT>();
pub const D5_C2F_LIST: [CoordToFeatureIndex; D5_C2F_COUNT] = make_c2f_list::<D5, D5_C2F_COUNT>();
pub const E5_C2F_LIST: [CoordToFeatureIndex; E5_C2F_COUNT] = make_c2f_list::<E5, E5_C2F_COUNT>();
pub const F5_C2F_LIST: [CoordToFeatureIndex; F5_C2F_COUNT] = make_c2f_list::<F5, F5_C2F_COUNT>();
pub const G5_C2F_LIST: [CoordToFeatureIndex; G5_C2F_COUNT] = make_c2f_list::<G5, G5_C2F_COUNT>();
pub const H5_C2F_LIST: [CoordToFeatureIndex; H5_C2F_COUNT] = make_c2f_list::<H5, H5_C2F_COUNT>();

pub const A6_C2F_LIST: [CoordToFeatureIndex; A6_C2F_COUNT] = make_c2f_list::<A6, A6_C2F_COUNT>();
pub const B6_C2F_LIST: [CoordToFeatureIndex; B6_C2F_COUNT] = make_c2f_list::<B6, B6_C2F_COUNT>();
pub const C6_C2F_LIST: [CoordToFeatureIndex; C6_C2F_COUNT] = make_c2f_list::<C6, C6_C2F_COUNT>();
pub const D6_C2F_LIST: [CoordToFeatureIndex; D6_C2F_COUNT] = make_c2f_list::<D6, D6_C2F_COUNT>();
pub const E6_C2F_LIST: [CoordToFeatureIndex; E6_C2F_COUNT] = make_c2f_list::<E6, E6_C2F_COUNT>();
pub const F6_C2F_LIST: [CoordToFeatureIndex; F6_C2F_COUNT] = make_c2f_list::<F6, F6_C2F_COUNT>();
pub const G6_C2F_LIST: [CoordToFeatureIndex; G6_C2F_COUNT] = make_c2f_list::<G6, G6_C2F_COUNT>();
pub const H6_C2F_LIST: [CoordToFeatureIndex; H6_C2F_COUNT] = make_c2f_list::<H6, H6_C2F_COUNT>();

pub const A7_C2F_LIST: [CoordToFeatureIndex; A7_C2F_COUNT] = make_c2f_list::<A7, A7_C2F_COUNT>();
pub const B7_C2F_LIST: [CoordToFeatureIndex; B7_C2F_COUNT] = make_c2f_list::<B7, B7_C2F_COUNT>();
pub const C7_C2F_LIST: [CoordToFeatureIndex; C7_C2F_COUNT] = make_c2f_list::<C7, C7_C2F_COUNT>();
pub const D7_C2F_LIST: [CoordToFeatureIndex; D7_C2F_COUNT] = make_c2f_list::<D7, D7_C2F_COUNT>();
pub const E7_C2F_LIST: [CoordToFeatureIndex; E7_C2F_COUNT] = make_c2f_list::<E7, E7_C2F_COUNT>();
pub const F7_C2F_LIST: [CoordToFeatureIndex; F7_C2F_COUNT] = make_c2f_list::<F7, F7_C2F_COUNT>();
pub const G7_C2F_LIST: [CoordToFeatureIndex; G7_C2F_COUNT] = make_c2f_list::<G7, G7_C2F_COUNT>();
pub const H7_C2F_LIST: [CoordToFeatureIndex; H7_C2F_COUNT] = make_c2f_list::<H7, H7_C2F_COUNT>();

pub const A8_C2F_LIST: [CoordToFeatureIndex; A8_C2F_COUNT] = make_c2f_list::<A8, A8_C2F_COUNT>();
pub const B8_C2F_LIST: [CoordToFeatureIndex; B8_C2F_COUNT] = make_c2f_list::<B8, B8_C2F_COUNT>();
pub const C8_C2F_LIST: [CoordToFeatureIndex; C8_C2F_COUNT] = make_c2f_list::<C8, C8_C2F_COUNT>();
pub const D8_C2F_LIST: [CoordToFeatureIndex; D8_C2F_COUNT] = make_c2f_list::<D8, D8_C2F_COUNT>();
pub const E8_C2F_LIST: [CoordToFeatureIndex; E8_C2F_COUNT] = make_c2f_list::<E8, E8_C2F_COUNT>();
pub const F8_C2F_LIST: [CoordToFeatureIndex; F8_C2F_COUNT] = make_c2f_list::<F8, F8_C2F_COUNT>();
pub const G8_C2F_LIST: [CoordToFeatureIndex; G8_C2F_COUNT] = make_c2f_list::<G8, G8_C2F_COUNT>();
pub const H8_C2F_LIST: [CoordToFeatureIndex; H8_C2F_COUNT] = make_c2f_list::<H8, H8_C2F_COUNT>();

pub const C2F_LISTS: [&[CoordToFeatureIndex]; 64] = [
    &A1_C2F_LIST,
    &B1_C2F_LIST,
    &C1_C2F_LIST,
    &D1_C2F_LIST,
    &E1_C2F_LIST,
    &F1_C2F_LIST,
    &G1_C2F_LIST,
    &H1_C2F_LIST,
    &A2_C2F_LIST,
    &B2_C2F_LIST,
    &C2_C2F_LIST,
    &D2_C2F_LIST,
    &E2_C2F_LIST,
    &F2_C2F_LIST,
    &G2_C2F_LIST,
    &H2_C2F_LIST,
    &A3_C2F_LIST,
    &B3_C2F_LIST,
    &C3_C2F_LIST,
    &D3_C2F_LIST,
    &E3_C2F_LIST,
    &F3_C2F_LIST,
    &G3_C2F_LIST,
    &H3_C2F_LIST,
    &A4_C2F_LIST,
    &B4_C2F_LIST,
    &C4_C2F_LIST,
    &D4_C2F_LIST,
    &E4_C2F_LIST,
    &F4_C2F_LIST,
    &G4_C2F_LIST,
    &H4_C2F_LIST,
    &A5_C2F_LIST,
    &B5_C2F_LIST,
    &C5_C2F_LIST,
    &D5_C2F_LIST,
    &E5_C2F_LIST,
    &F5_C2F_LIST,
    &G5_C2F_LIST,
    &H5_C2F_LIST,
    &A6_C2F_LIST,
    &B6_C2F_LIST,
    &C6_C2F_LIST,
    &D6_C2F_LIST,
    &E6_C2F_LIST,
    &F6_C2F_LIST,
    &G6_C2F_LIST,
    &H6_C2F_LIST,
    &A7_C2F_LIST,
    &B7_C2F_LIST,
    &C7_C2F_LIST,
    &D7_C2F_LIST,
    &E7_C2F_LIST,
    &F7_C2F_LIST,
    &G7_C2F_LIST,
    &H7_C2F_LIST,
    &A8_C2F_LIST,
    &B8_C2F_LIST,
    &C8_C2F_LIST,
    &D8_C2F_LIST,
    &E8_C2F_LIST,
    &F8_C2F_LIST,
    &G8_C2F_LIST,
    &H8_C2F_LIST,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c2f_count() {
        let a1_c2f_list = make_c2f_list::<A1, A1_C2F_COUNT>();

        assert_eq!(a1_c2f_list.len(), A1_C2F_COUNT);
    }
}
