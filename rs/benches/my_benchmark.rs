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
    c.bench_function("fib 15", |b| b.iter(|| fibonacci(black_box(15))));
}

pub fn sort_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0., 1.);
    let v: Vec<f64> = (0..1_000_000).map(|_| rng.sample(&range)).collect();

    c.bench_function("sort", |b| {
        b.iter(|| {
            let mut w = v.clone();
            w.sort_by(|a, b| a.partial_cmp(b).unwrap());
        })
    });
}

pub fn par_sort_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0., 1.);
    let v: Vec<f64> = (0..1_000_000).map(|_| rng.sample(&range)).collect();

    c.bench_function("par_sort", |b| {
        b.iter(|| {
            let mut w = v.clone();
            w.par_sort_by(|a, b| a.partial_cmp(b).unwrap());
        })
    });
}

criterion_group!(benches, criterion_benchmark, sort_benchmark, par_sort_benchmark);
criterion_main!(benches);
