use criterion::{black_box, Criterion, criterion_group, criterion_main};
use rand::{distributions::Uniform, Rng};
use rayon::prelude::*;

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
            black_box(w.par_sort_by(|a, b| a.partial_cmp(b).unwrap()));
        })
    });
}

criterion_group!(sort, 
    sort_benchmark, 
    par_sort_benchmark,
);

criterion_main!(sort);
