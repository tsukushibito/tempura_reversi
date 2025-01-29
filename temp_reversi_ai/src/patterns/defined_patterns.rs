use super::PatternGroup;
use temp_reversi_core::*;

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
        (A2 | B2 | C2 | D2 | E2 | F2 | G2 | H2, "Line Pattern 1"),
        (A3 | B3 | C3 | D3 | E3 | F3 | G3 | H3, "Line Pattern 2"),
        (A4 | B4 | C4 | D4 | E4 | F4 | G4 | H4, "Line Pattern 3"),
        (
            A1 | B1 | A2 | B2 | C3 | D4 | E5 | F6 | G7 | H8,
            "Diagonal Pattern 1",
        ),
        (B1 | C2 | D3 | E4 | F5 | G6 | H7, "Diagonal Pattern 2"),
        (C1 | D2 | E3 | F4 | G5 | H6, "Diagonal Pattern 3"),
        (D1 | E2 | F3 | G4 | H5, "Diagonal Pattern 4"),
        (
            A1 | B1 | C1 | D1 | E1 | F1 | G1 | H1 | B2 | G2,
            "Edge Pattern 1",
        ),
        (
            A1 | B1 | C1 | D1 | E1 | F1 | G1 | H1 | C2 | F2,
            "Edge Pattern 2",
        ),
        (
            A1 | C1 | D1 | E1 | F1 | H1 | C2 | D2 | E2 | F2,
            "Edge Pattern 3",
        ),
        (
            C1 | D1 | E1 | F1 | D2 | E2 | C3 | D3 | E3 | F3,
            "Edge Pattern 4",
        ),
        (
            A1 | B1 | C1 | A2 | B2 | C2 | A3 | B3 | C4,
            "Corner Pattern 1",
        ),
        (
            A1 | B1 | C1 | D1 | A2 | B2 | C2 | A3 | B3 | D1,
            "Corner Pattern 2",
        ),
        (
            A1 | B1 | C1 | D1 | E1 | A2 | B2 | A3 | A4 | A5,
            "Corner Pattern 3",
        ),
        (
            A1 | B1 | A2 | B2 | C2 | B3 | C3 | D3 | C4 | D4,
            "Corner Pattern 4",
        ),
        (
            A1 | B1 | A2 | B2 | C2 | D2 | B3 | C3 | B4 | D4,
            "Corner Pattern 5",
        ),
    ];

    mask_and_names
        .iter()
        .map(|(mask, name)| {
            let state_scores = vec![vec![0; 3_usize.pow(mask.count_ones())]; 60];
            PatternGroup::new(*mask, state_scores, Some(*name))
        })
        .collect()
}
