use std::cmp::Reverse;

use temp_game_ai::{
    searcher::{NegaMax, Searcher},
    util::perft,
};
use temp_reversi_ai::evaluator::{ReversiState, SimpleEvaluator};

#[test]
fn test_nega_max() {
    let evaluator = SimpleEvaluator;
    let mut nega_max = NegaMax::new(evaluator);

    let state = ReversiState::default();

    let _m = nega_max.search(&state, 3);

    let nodes = perft(&state, 3, false);

    assert_eq!(nega_max.visited_nodes, nodes);
}
