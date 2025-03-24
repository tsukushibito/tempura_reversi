use temp_game_ai::{
    searcher::{NegaMax, Searcher},
    util::perft,
};
use temp_reversi_ai::{evaluator::SimpleEvaluator, ReversiState};

#[test]
fn test_nega_max() {
    let evaluator = SimpleEvaluator;
    let mut nega_max = NegaMax::new(evaluator);

    let mut state = ReversiState::default();

    let _m = nega_max.search(&mut state, 4);

    let nodes = 1 + (1..=4).map(|d| perft(&mut state, d)).sum::<usize>();

    assert_eq!(nega_max.visited_nodes, nodes);
}
