// Benchmarks are like integration tests: out-of-the crate, public API only
// Run with: cargo bench -p mylib1
use criterion::{Criterion, criterion_group, criterion_main};
use mylib1::f4;

fn bench_f4(c: &mut Criterion) {
    c.bench_function("f4", |b| b.iter(f4));
}

criterion_group!(benches, bench_f4);
criterion_main!(benches);
