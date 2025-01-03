use std::{
    fs::File,
    io::{Read, Write},
};

use crate::{BitBoard, DynResult, Pattern, Position};

use serde::{Deserialize, Serialize};

use super::sparse_feature::SparseFeature;

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    patterns: Vec<Pattern>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            patterns: generate_patterns(),
        }
    }
}

impl Model {
    pub fn new(patterns: &[Pattern]) -> Self {
        Self {
            patterns: patterns.to_vec(),
        }
    }

    pub fn load(file_path: &str) -> DynResult<Self> {
        let mut file = File::open(file_path)?;
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        let model: Self = bincode::deserialize(&buf)?;

        Ok(model)
    }

    pub fn save(&self, file_path: &str) -> DynResult<()> {
        let mut file = File::open(file_path)?;
        let serialized = bincode::serialize(self)?;
        file.write_all(&serialized)?;
        file.flush()?;
        Ok(())
    }

    pub fn patterns(&self) -> &Vec<Pattern> {
        &self.patterns
    }

    pub fn feature(&self, board: &BitBoard) -> SparseFeature {
        self.patterns
            .iter()
            .fold(SparseFeature::default(), |acc, pattern| {
                acc.concat(&pattern.feature(board)).unwrap_or_default()
            })
    }

    pub fn values(&self) -> Vec<f32> {
        self.patterns
            .iter()
            .flat_map(|pattern| pattern.values.iter().copied())
            .collect()
    }

    pub fn evaluate(&self, board: &BitBoard) -> f32 {
        self.patterns
            .iter()
            .map(|pattern| pattern.value(board))
            .sum()
    }
}

const LINE_A: [Position; 8] = [
    Position::A2,
    Position::B2,
    Position::C2,
    Position::D2,
    Position::E2,
    Position::F2,
    Position::G2,
    Position::H2,
];

const LINE_B: [Position; 8] = [
    Position::A3,
    Position::B3,
    Position::C3,
    Position::D3,
    Position::E3,
    Position::F3,
    Position::G3,
    Position::H3,
];

const LINE_C: [Position; 8] = [
    Position::A4,
    Position::B4,
    Position::C4,
    Position::D4,
    Position::E4,
    Position::F4,
    Position::G4,
    Position::H4,
];

const DIAGONAL_A: [Position; 5] = [
    Position::D1,
    Position::E2,
    Position::F3,
    Position::G4,
    Position::H5,
];

const DIAGONAL_B: [Position; 6] = [
    Position::C1,
    Position::D2,
    Position::E3,
    Position::F4,
    Position::G5,
    Position::H6,
];

const DIAGONAL_C: [Position; 7] = [
    Position::B1,
    Position::C2,
    Position::D3,
    Position::E4,
    Position::F5,
    Position::G6,
    Position::H7,
];

const DIAGONAL_D: [Position; 10] = [
    Position::A1,
    Position::B1,
    Position::A2,
    Position::B2,
    Position::C3,
    Position::D4,
    Position::E5,
    Position::F6,
    Position::G7,
    Position::H8,
];

const CORNER_A: [Position; 9] = [
    Position::A1,
    Position::A2,
    Position::A3,
    Position::B1,
    Position::B2,
    Position::B3,
    Position::C1,
    Position::C2,
    Position::C3,
];

const CORNER_B: [Position; 10] = [
    Position::A1,
    Position::B1,
    Position::C1,
    Position::D1,
    Position::A2,
    Position::B2,
    Position::C2,
    Position::A3,
    Position::B3,
    Position::A4,
];

const CORNER_C: [Position; 10] = [
    Position::A1,
    Position::B1,
    Position::A2,
    Position::B2,
    Position::C2,
    Position::B3,
    Position::C3,
    Position::D3,
    Position::C4,
    Position::D4,
];

const CORNER_D: [Position; 10] = [
    Position::A1,
    Position::B1,
    Position::C1,
    Position::D1,
    Position::E1,
    Position::A2,
    Position::B2,
    Position::A3,
    Position::A4,
    Position::A5,
];

const CORNER_E: [Position; 10] = [
    Position::A1,
    Position::B1,
    Position::A2,
    Position::B2,
    Position::C2,
    Position::D2,
    Position::B2,
    Position::C3,
    Position::D2,
    Position::D4,
];

const EDGE_A: [Position; 10] = [
    Position::A1,
    Position::B1,
    Position::C1,
    Position::D1,
    Position::E1,
    Position::F1,
    Position::G1,
    Position::H1,
    Position::B2,
    Position::G2,
];

const EDGE_B: [Position; 10] = [
    Position::A1,
    Position::C1,
    Position::D1,
    Position::E1,
    Position::F1,
    Position::H1,
    Position::C2,
    Position::D2,
    Position::E2,
    Position::F2,
];

const EDGE_C: [Position; 10] = [
    Position::A1,
    Position::B1,
    Position::C1,
    Position::D1,
    Position::E1,
    Position::F1,
    Position::G1,
    Position::H1,
    Position::C2,
    Position::F2,
];

const EDGE_D: [Position; 10] = [
    Position::C1,
    Position::D1,
    Position::E1,
    Position::F1,
    Position::D2,
    Position::E2,
    Position::A3,
    Position::B3,
    Position::C3,
    Position::D3,
];

fn generate_patterns() -> Vec<Pattern> {
    vec![
        Pattern::from_positions(0, &LINE_A),
        Pattern::from_positions(1, &LINE_B),
        Pattern::from_positions(2, &LINE_C),
        Pattern::from_positions(4, &DIAGONAL_A),
        Pattern::from_positions(5, &DIAGONAL_B),
        Pattern::from_positions(6, &DIAGONAL_C),
        Pattern::from_positions(7, &DIAGONAL_D),
        Pattern::from_positions(3, &CORNER_A),
        Pattern::from_positions(9, &CORNER_B),
        Pattern::from_positions(11, &CORNER_C),
        Pattern::from_positions(13, &CORNER_D),
        Pattern::from_positions(14, &CORNER_E),
        Pattern::from_positions(8, &EDGE_A),
        Pattern::from_positions(10, &EDGE_B),
        Pattern::from_positions(12, &EDGE_C),
        Pattern::from_positions(15, &EDGE_D),
    ]
}
