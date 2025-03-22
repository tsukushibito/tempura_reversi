use temp_game_ai::util::perft;
use temp_reversi_ai::ReversiState;

#[test]
fn test_reversi_perft() {
    let state = ReversiState::default();
    println!("Running perft tests for Reversi...");

    println!("Depth 1");
    let nodes = perft(&state, 1);
    assert_eq!(nodes, 4);

    println!("Depth 2");
    let nodes = perft(&state, 2);
    assert_eq!(nodes, 12);

    println!("Depth 3");
    let nodes = perft(&state, 3);
    assert_eq!(nodes, 56);

    println!("Depth 4");
    let nodes = perft(&state, 4);
    assert_eq!(nodes, 244);

    println!("Depth 5");
    let nodes = perft(&state, 5);
    assert_eq!(nodes, 1396);

    println!("Depth 6");
    let nodes = perft(&state, 6);
    assert_eq!(nodes, 8200);

    println!("Depth 7");
    let nodes = perft(&state, 7);
    assert_eq!(nodes, 55092);

    println!("Depth 8");
    let nodes = perft(&state, 8);
    assert_eq!(nodes, 390216);

    println!("Depth 9");
    let nodes = perft(&state, 9);
    assert_eq!(nodes, 3005288);

    println!("Depth 10");
    let nodes = perft(&state, 10);
    assert_eq!(nodes, 24571284);

    println!("Depth 11");
    let nodes = perft(&state, 11);
    assert_eq!(nodes, 212258800);

    println!("Depth 12");
    let nodes = perft(&state, 12);
    assert_eq!(nodes, 1939886636);

    // Skip further depths as they take too long to run

    // println!("Depth 13");
    // let nodes = perft(&state, 13);
    // assert_eq!(nodes, 18429641748);

    // println!("Depth 14");
    // let nodes = perft(&state, 14);
    // assert_eq!(nodes, 184042084512);

    // println!("Depth 15");
    // let nodes = perft(&state, 15);
    // assert_eq!(nodes, 1891832540064);
}
