use std::{fs::File, io::Read, path::Path};

use crate::{
    bit_board::BitBoard,
    ml::{Model, ModelInput},
    Board, Color, Pattern, Position, ResultBoxErr, SparseVector,
};

use super::{Evaluator, TestEvaluator};

#[derive(Debug)]
pub struct TempuraEvaluator {
    pub test_evaluator: TestEvaluator,
    pub patterns: Vec<Pattern>,
    pub model: Model,
}

impl Default for TempuraEvaluator {
    fn default() -> Self {
        let patterns = generate_patterns();
        let input_size = patterns.iter().map(|p| p.state_count()).sum();
        let model: Model = Model::new(input_size);
        let test_evaluator = TestEvaluator::default();

        Self {
            patterns,
            model,
            test_evaluator,
        }
    }
}

impl TempuraEvaluator {
    pub fn load<P: AsRef<Path>>(file_path: P) -> ResultBoxErr<Self> {
        let model: Model = Model::load_model(file_path)?;
        let patterns = generate_patterns();
        let test_evaluator = TestEvaluator::default();

        Ok(Self {
            patterns,
            model,
            test_evaluator,
        })
    }

    pub fn patterns(&self) -> &Vec<Pattern> {
        &self.patterns
    }

    pub fn feature(&self, board: &BitBoard) -> SparseVector {
        self.patterns
            .iter()
            .fold(SparseVector::default(), |acc, pattern| {
                acc.concat(&pattern.feature(board)).unwrap_or_default()
            })
    }

    pub fn feature_size(&self) -> usize {
        self.patterns.iter().map(|p| p.state_count()).sum()
    }
}

impl Evaluator for TempuraEvaluator {
    fn evaluate(&self, board: &BitBoard, color: Color) -> i32 {
        let phase = std::cmp::min(60 - board.empty_count() - 1, 59);
        if phase < 20 {
            self.test_evaluator.evaluate(board, color)
        } else {
            let feature = self.feature(board);
            let input = ModelInput { phase, feature };
            let output = self.model.forward(&[input]);
            let value = output[0] as i32;
            match color {
                Color::Black => value,
                Color::White => -value,
            }
        }
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
