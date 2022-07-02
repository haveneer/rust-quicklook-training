use criterion::{black_box, criterion_group, criterion_main, Criterion};

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

fn fold_factorial(n: u64) -> u64 {
    (1..=n).fold(1, |acc, i| acc * i)
}

pub fn imperative_factorial_benchmark(c: &mut Criterion) {
    c.bench_function("imperative_factorial", |b| {
        b.iter(|| imperative_factorial(black_box(UPPER_BOUND)))
    });
}

pub fn recursive_factorial_benchmark(c: &mut Criterion) {
    c.bench_function("recursive_factorial", |b| {
        b.iter(|| recursive_factorial(black_box(UPPER_BOUND)))
    });
}

pub fn fold_factorial_benchmark(c: &mut Criterion) {
    c.bench_function("fold_factorial", |b| {
        b.iter(|| fold_factorial(black_box(UPPER_BOUND)))
    });
}

criterion_group!(
    factorial,
    imperative_factorial_benchmark,
    recursive_factorial_benchmark,
    fold_factorial_benchmark
);

criterion_main!(factorial);
