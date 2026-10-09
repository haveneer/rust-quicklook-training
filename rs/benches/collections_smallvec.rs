//! Many short-lived small vectors: Vec (always on the heap), SmallVec (inline up to
//! N = 8, then on the heap) and ArrayVec (inline, fixed capacity 8).
//! k = number of elements pushed in each vector (the sweep).
//!
//! cargo bench --bench collections_smallvec
//! venv/bin/python benches/plot_figures.py --only collections
use arrayvec::ArrayVec;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use smallvec::SmallVec;

const VECTORS: u32 = 10_000;
const LENGTHS: [u32; 6] = [1, 2, 4, 8, 16, 32];

fn bench_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("smallvec");
    for k in LENGTHS {
        let id = |name| BenchmarkId::new(name, k);
        // Build a vector of k items, use it, drop it; VECTORS times
        group.bench_function(id("vec"), |b| {
            b.iter(|| {
                let mut total = 0;
                for i in 0..VECTORS {
                    let mut v: Vec<u32> = Vec::new();
                    v.extend((0..black_box(k)).map(|j| i ^ j));
                    total += v.iter().sum::<u32>();
                }
                total
            })
        });
        group.bench_function(id("smallvec_8"), |b| {
            b.iter(|| {
                let mut total = 0;
                for i in 0..VECTORS {
                    let mut v: SmallVec<[u32; 8]> = SmallVec::new();
                    v.extend((0..black_box(k)).map(|j| i ^ j));
                    total += v.iter().sum::<u32>();
                }
                total
            })
        });
        if k <= 8 {
            group.bench_function(id("arrayvec_8"), |b| {
                b.iter(|| {
                    let mut total = 0;
                    for i in 0..VECTORS {
                        let mut v: ArrayVec<u32, 8> = ArrayVec::new();
                        v.extend((0..black_box(k)).map(|j| i ^ j));
                        total += v.iter().sum::<u32>();
                    }
                    total
                })
            });
        }
    }
    group.finish();
}

criterion_group!(benches, bench_small);
criterion_main!(benches);
