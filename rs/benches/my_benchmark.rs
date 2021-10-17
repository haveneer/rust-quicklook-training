use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{distributions::Uniform, Rng};
use rayon::prelude::*;

#[inline]
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

pub fn sort_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0., 1.);
    let v: Vec<f64> = (0..10_000_000).map(|_| rng.sample(&range)).collect();

    c.bench_function("sort 10", |b| {
        b.iter(|| {
            let mut w = v.clone();
            w.par_sort_by(|a, b| a.partial_cmp(b).unwrap());
        })
    });
}

criterion_group!(benches, criterion_benchmark, sort_benchmark);
criterion_main!(benches);
