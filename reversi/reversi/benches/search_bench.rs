use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_test(_c: &mut Criterion) {}

criterion_group!(benches, benchmark_test,);
criterion_main!(benches);
