use temp_reversi_core::Position;

use super::PatternGroup;

/// Returns a predefined set of `PatternGroup` instances used for board evaluation.
///
/// These patterns represent various strategic board structures, including:
/// - Line patterns (horizontal rows)
/// - Diagonal patterns
/// - Edge patterns
/// - Corner patterns
///
/// Each pattern is associated with a bitmask and a descriptive name.
/// The corresponding state scores are initialized with zero values and can be adjusted later.
///
/// # Returns
/// * `Vec<PatternGroup>` - A collection of predefined pattern groups.
pub fn get_predefined_patterns() -> Vec<PatternGroup> {
    let mask_and_names = [
        (
            Position::A2
                | Position::B2
                | Position::C2
                | Position::D2
                | Position::E2
                | Position::F2
                | Position::G2
                | Position::H2,
            "Line Pattern 1",
        ),
        (
            Position::A3
                | Position::B3
                | Position::C3
                | Position::D3
                | Position::E3
                | Position::F3
                | Position::G3
                | Position::H3,
            "Line Pattern 2",
        ),
        (
            Position::A4
                | Position::B4
                | Position::C4
                | Position::D4
                | Position::E4
                | Position::F4
                | Position::G4
                | Position::H4,
            "Line Pattern 3",
        ),
        (
            Position::A1
                | Position::B1
                | Position::A2
                | Position::B2
                | Position::C3
                | Position::D4
                | Position::E5
                | Position::F6
                | Position::G7
                | Position::H8,
            "Diagonal Pattern 1",
        ),
        (
            Position::B1
                | Position::C2
                | Position::D3
                | Position::E4
                | Position::F5
                | Position::G6
                | Position::H7,
            "Diagonal Pattern 2",
        ),
        (
            Position::C1 | Position::D2 | Position::E3 | Position::F4 | Position::G5 | Position::H6,
            "Diagonal Pattern 3",
        ),
        (
            Position::D1 | Position::E2 | Position::F3 | Position::G4 | Position::H5,
            "Diagonal Pattern 4",
        ),
        (
            Position::A1
                | Position::B1
                | Position::C1
                | Position::D1
                | Position::E1
                | Position::F1
                | Position::G1
                | Position::H1
                | Position::B2
                | Position::G2,
            "Edge Pattern 1",
        ),
        (
            Position::A1
                | Position::B1
                | Position::C1
                | Position::D1
                | Position::E1
                | Position::F1
                | Position::G1
                | Position::H1
                | Position::C2
                | Position::F2,
            "Edge Pattern 2",
        ),
        (
            Position::A1
                | Position::C1
                | Position::D1
                | Position::E1
                | Position::F1
                | Position::H1
                | Position::C2
                | Position::D2
                | Position::E2
                | Position::F2,
            "Edge Pattern 3",
        ),
        (
            Position::C1
                | Position::D1
                | Position::E1
                | Position::F1
                | Position::D2
                | Position::E2
                | Position::C3
                | Position::D3
                | Position::E3
                | Position::F3,
            "Edge Pattern 4",
        ),
        (
            Position::A1
                | Position::B1
                | Position::C1
                | Position::A2
                | Position::B2
                | Position::C2
                | Position::A3
                | Position::B3
                | Position::C4,
            "Corner Pattern 1",
        ),
        (
            Position::A1
                | Position::B1
                | Position::C1
                | Position::D1
                | Position::A2
                | Position::B2
                | Position::C2
                | Position::A3
                | Position::B3
                | Position::D1,
            "Corner Pattern 2",
        ),
        (
            Position::A1
                | Position::B1
                | Position::C1
                | Position::D1
                | Position::E1
                | Position::A2
                | Position::B2
                | Position::A3
                | Position::A4
                | Position::A5,
            "Corner Pattern 3",
        ),
        (
            Position::A1
                | Position::B1
                | Position::A2
                | Position::B2
                | Position::C2
                | Position::B3
                | Position::C3
                | Position::D3
                | Position::C4
                | Position::D4,
            "Corner Pattern 4",
        ),
        (
            Position::A1
                | Position::B1
                | Position::A2
                | Position::B2
                | Position::C2
                | Position::D2
                | Position::B3
                | Position::C3
                | Position::B4
                | Position::D4,
            "Corner Pattern 5",
        ),
    ];

    mask_and_names
        .iter()
        .map(|(mask, name)| {
            let state_scores = vec![vec![0.0; 3_usize.pow(mask.count_ones())]; 60];
            PatternGroup::new(*mask, state_scores, Some(*name))
        })
        .collect()
}
