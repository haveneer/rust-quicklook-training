use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

const UPPER_BOUND: u64 = 17;

fn imperative_factorial(mut n: u64) -> u64 {
    let mut r = 1;
    while n > 1 {
        r *= n;
        n -= 1;
    }
    r
}

fn recursive_factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n => n * recursive_factorial(n - 1),
    }
}

#[allow(clippy::unnecessary_fold)] // on purpose: product() would do the same
fn fold_factorial(n: u64) -> u64 {
    (1..=n).fold(1, |acc, i| acc * i)
}

// One group: criterion compares its functions (summary and violin plot)
pub fn factorial_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("factorial");
    group.bench_function("imperative", |b| {
        b.iter(|| imperative_factorial(black_box(UPPER_BOUND)))
    });
    group.bench_function("recursive", |b| {
        b.iter(|| recursive_factorial(black_box(UPPER_BOUND)))
    });
    group.bench_function("fold", |b| {
        b.iter(|| fold_factorial(black_box(UPPER_BOUND)))
    });
    group.finish();
}

criterion_group!(factorial, factorial_benchmark);
criterion_main!(factorial);
