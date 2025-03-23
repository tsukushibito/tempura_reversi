use temp_game_ai::util::{perft, perft};
use temp_reversi_ai::{ReversiState, ReversiState2};

fn measure_time<F: FnOnce() -> R, R>(f: F) -> (R, std::time::Duration) {
    let start = std::time::Instant::now();
    let result = f();
    let elapsed = start.elapsed();
    println!("Elapsed: {:?}", elapsed);
    (result, elapsed)
}

fn test_reversi_perft2() {
    println!("Running perft2 tests for Reversi...");
    let mut state = ReversiState2::default();

    measure_time(|| {
        println!("Depth 1");
        let nodes = perft(&mut state, 1);
        assert_eq!(nodes, 4);
    });

    measure_time(|| {
        println!("Depth 2");
        let nodes = perft(&mut state, 2);
        assert_eq!(nodes, 12);
    });

    measure_time(|| {
        println!("Depth 3");
        let nodes = perft(&mut state, 3);
        assert_eq!(nodes, 56);
    });

    measure_time(|| {
        println!("Depth 4");
        let nodes = perft(&mut state, 4);
        assert_eq!(nodes, 244);
    });

    measure_time(|| {
        println!("Depth 5");
        let nodes = perft(&mut state, 5);
        assert_eq!(nodes, 1396);
    });

    measure_time(|| {
        println!("Depth 6");
        let nodes = perft(&mut state, 6);
        assert_eq!(nodes, 8200);
    });

    measure_time(|| {
        println!("Depth 7");
        let nodes = perft(&mut state, 7);
        assert_eq!(nodes, 55092);
    });

    measure_time(|| {
        println!("Depth 8");
        let nodes = perft(&mut state, 8);
        assert_eq!(nodes, 390216);
    });

    measure_time(|| {
        println!("Depth 9");
        let nodes = perft(&mut state, 9);
        assert_eq!(nodes, 3005288);
    });

    measure_time(|| {
        println!("Depth 10");
        let nodes = perft(&mut state, 10);
        assert_eq!(nodes, 24571284);
    });

    // Skip further depths as they take too long to run
    // measure_time(|| {
    //     println!("Depth 11");
    //     let nodes = perft2(&mut state, 11);
    //     assert_eq!(nodes, 212258800);
    // });

    // measure_time(|| {
    //     println!("Depth 12");
    //     let nodes = perft2(&state, 12);
    //     assert_eq!(nodes, 1939886636);
    // });

    // println!("Depth 13");
    // let nodes = perft2(&state, 13);
    // assert_eq!(nodes, 18429641748);

    // println!("Depth 14");
    // let nodes = perft2(&state, 14);
    // assert_eq!(nodes, 184042084512);

    // println!("Depth 15");
    // let nodes = perft2(&state, 15);
    // assert_eq!(nodes, 1891832540064);
}
