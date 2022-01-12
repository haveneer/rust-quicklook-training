use criterion::{black_box, Criterion, criterion_group, criterion_main};

#[inline]
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 15", |b| b.iter(|| fibonacci(black_box(15))));
}

criterion_group!(fibonacci, 
    criterion_benchmark, 
);

criterion_main!(fibonacci);
