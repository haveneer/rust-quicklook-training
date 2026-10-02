//! Measurements behind the "Techniques d'optimisation avancées" slides.
//!
//! cargo bench --bench optimisations
//! venv/bin/python benches/plot_figures.py --only optimisations
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const N: usize = 1 << 16;

/// Deterministic pseudo-random bytes (no rand: reproducible measurements)
fn data() -> Vec<u8> {
    let mut x: u32 = 0x2545_F491;
    (0..N)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            (x >> 24) as u8
        })
        .collect()
}

/// A data-dependent branch the compiler keeps as a branch:
/// each taken iteration has a side effect (an indexed write)
fn count_large(values: &[u8], hits: &mut [u32; 256]) -> u32 {
    let mut n = 0;
    for &v in values {
        if v >= 128 {
            hits[v as usize] += 1;
            n += 1;
        }
    }
    n
}

/// Same result without a branch: the condition becomes arithmetic
fn count_large_branchless(values: &[u8], hits: &mut [u32; 256]) -> u32 {
    let mut n = 0;
    for &v in values {
        let taken = u32::from(v >= 128);
        hits[v as usize] += taken;
        n += taken;
    }
    n
}

fn bench_branch(c: &mut Criterion) {
    let random = data();
    let mut sorted = random.clone();
    sorted.sort_unstable();
    let mut group = c.benchmark_group("branch_prediction");
    group.bench_function("random", |b| {
        b.iter(|| count_large(black_box(&random), &mut [0; 256]))
    });
    group.bench_function("sorted", |b| {
        b.iter(|| count_large(black_box(&sorted), &mut [0; 256]))
    });
    group.bench_function("random_branchless", |b| {
        b.iter(|| count_large_branchless(black_box(&random), &mut [0; 256]))
    });
    group.finish();
}

/// Opaque to the caller: one real call per element
#[inline(never)]
fn add_never(a: u32, b: u32) -> u32 {
    a.wrapping_add(b)
}

/// Inlined into the loop, which LLVM can then vectorise
#[inline]
fn add_inline(a: u32, b: u32) -> u32 {
    a.wrapping_add(b)
}

fn bench_inlining(c: &mut Criterion) {
    let xs: Vec<u32> = (0..N as u32).collect();
    let mut group = c.benchmark_group("inlining");
    group.bench_function("inlined", |b| {
        b.iter(|| black_box(&xs).iter().fold(0, |acc, &x| add_inline(acc, x)))
    });
    group.bench_function("inline_never", |b| {
        b.iter(|| black_box(&xs).iter().fold(0, |acc, &x| add_never(acc, x)))
    });
    group.finish();
}

criterion_group!(benches, bench_branch, bench_inlining);
criterion_main!(benches);
