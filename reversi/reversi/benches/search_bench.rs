use criterion::{black_box, criterion_group, criterion_main, Criterion};
use reversi::{
    ai::{
        evaluate::simple_evaluate,
        search::{Negaalpha, Negamax},
        GameState,
    },
    bit_board::BitBoard,
    board::Board,
    Color,
};

// ベンチマーク用の深さを設定
const DEPTH: u8 = 5;

// negamaxのベンチマーク
fn benchmark_negamax(c: &mut Criterion) {
    c.bench_function(&format!("negamax depth {}", DEPTH), |b| {
        b.iter(|| {
            let board = BitBoard::new();
            let state = GameState::new(board, Color::Black);
            let mut negamax = Negamax::new(simple_evaluate);
            let r = negamax.search(&state, DEPTH);
            black_box(r);
        })
    });
}

// ムーブオーダリングなしのnegaalphaのベンチマーク
fn benchmark_negaalpha_no_move_ordering(c: &mut Criterion) {
    c.bench_function(
        &format!("negaalpha no move ordering depth {}", DEPTH),
        |b| {
            b.iter(|| {
                let board = BitBoard::new();
                let state = GameState::new(board, Color::Black);
                let mut negaalpha = Negaalpha::new(simple_evaluate);
                negaalpha.set_move_ordering(false);
                let alpha = i32::MIN + 1;
                let beta = i32::MAX;
                black_box(negaalpha.search(&state, DEPTH, alpha, beta));
            })
        },
    );
}

// ムーブオーダリングありのnegaalphaのベンチマーク
fn benchmark_negaalpha_with_move_ordering(c: &mut Criterion) {
    c.bench_function(
        &format!("negaalpha with move ordering depth {}", DEPTH),
        |b| {
            b.iter(|| {
                let board = BitBoard::new();
                let state = GameState::new(board, Color::Black);
                let mut negaalpha = Negaalpha::new(simple_evaluate);
                negaalpha.set_move_ordering(true);
                let alpha = i32::MIN + 1;
                let beta = i32::MAX;
                black_box(negaalpha.search(&state, DEPTH, alpha, beta));
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
