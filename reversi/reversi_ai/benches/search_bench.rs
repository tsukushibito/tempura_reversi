use criterion::{black_box, criterion_group, criterion_main, Criterion};
use reversi_ai::{
    evaluate::simple_evaluate,
    search::{Negaalpha, Negamax},
    GameState,
};
use reversi_core::{array_board::ArrayBoard, board::Board, Color};

// ベンチマーク用の深さを設定
const DEPTH: usize = 9;

// negamaxのベンチマーク
fn benchmark_negamax(c: &mut Criterion) {
    // ボードを初期化
    let board = ArrayBoard::new();
    let state = GameState::new(board, Color::Black);

    let mut negamax = Negamax::new(simple_evaluate);

    c.bench_function(&format!("negamax depth {}", DEPTH), |b| {
        b.iter(|| {
            let result = negamax.search(&state, DEPTH);
            black_box(result);
        })
    });
}

// ムーブオーダリングなしのnegaalphaのベンチマーク
fn benchmark_negaalpha_no_move_ordering(c: &mut Criterion) {
    // ボードを初期化
    let board = ArrayBoard::new();
    let state = GameState::new(board, Color::Black);

    // ムーブオーダリングを無効化したNegaalphaを作成
    let mut negaalpha = Negaalpha::new(simple_evaluate);
    negaalpha.set_move_ordering(false);

    c.bench_function(
        &format!("negaalpha no move ordering depth {}", DEPTH),
        |b| {
            b.iter(|| {
                let alpha = i32::MIN + 1;
                let beta = i32::MAX;
                let result = negaalpha.search(&state, DEPTH, alpha, beta);
                black_box(result);
            })
        },
    );
}

// ムーブオーダリングありのnegaalphaのベンチマーク
fn benchmark_negaalpha_with_move_ordering(c: &mut Criterion) {
    // ボードを初期化
    let board = ArrayBoard::new();
    let state = GameState::new(board, Color::Black);

    // ムーブオーダリングを有効化したNegaalphaを作成
    let mut negaalpha = Negaalpha::new(simple_evaluate);
    negaalpha.set_move_ordering(true);

    c.bench_function(
        &format!("negaalpha with move ordering depth {}", DEPTH),
        |b| {
            b.iter(|| {
                let alpha = i32::MIN + 1;
                let beta = i32::MAX;
                let result = negaalpha.search(&state, DEPTH, alpha, beta);
                black_box(result);
            })
        },
    );
}

criterion_group!(
    benches,
    benchmark_negamax,
    benchmark_negaalpha_no_move_ordering,
    benchmark_negaalpha_with_move_ordering,
);
criterion_main!(benches);
