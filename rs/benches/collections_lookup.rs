//! Membership lookups depending on the collection size: linear scan of a Vec,
//! binary search in a sorted Vec, HashMap and BTreeMap.
//!
//! cargo bench --bench collections_lookup
//! venv/bin/python benches/plot_figures.py --only collections
use criterion::measurement::WallTime;
use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion,
};
use std::collections::{BTreeMap, HashMap};

const QUERIES: usize = 1000;

/// Scrambled but deterministic keys (no rand: reproducible measurements)
fn key(i: usize) -> u64 {
    (i as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 16
}

/// Generic, so each lookup is inlined: no indirect call skewing the small sizes
fn run(
    group: &mut BenchmarkGroup<WallTime>,
    name: &str,
    n: usize,
    queries: &[u64],
    found: impl Fn(u64) -> bool,
) {
    group.bench_with_input(BenchmarkId::new(name, n), &n, |b, _| {
        b.iter(|| queries.iter().filter(|&&q| found(black_box(q))).count())
    });
}

fn bench_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("collections_lookup");
    for n in [8, 64, 512, 4096, 32768] {
        let keys: Vec<u64> = (0..n).map(key).collect();
        // Half of the queries hit, half miss
        let queries: Vec<u64> = (0..QUERIES)
            .map(|i| key(if i % 2 == 0 { i % n } else { n + i }))
            .collect();
        let mut sorted = keys.clone();
        sorted.sort_unstable();
        let hash: HashMap<u64, ()> = keys.iter().map(|&k| (k, ())).collect();
        let btree: BTreeMap<u64, ()> = keys.iter().map(|&k| (k, ())).collect();

        run(&mut group, "vec_contains", n, &queries, |q| {
            keys.contains(&q)
        });
        run(&mut group, "vec_binary_search", n, &queries, |q| {
            sorted.binary_search(&q).is_ok()
        });
        run(&mut group, "hashmap", n, &queries, |q| {
            hash.contains_key(&q)
        });
        run(&mut group, "btreemap", n, &queries, |q| {
            btree.contains_key(&q)
        });
    }
    group.finish();
}

criterion_group!(benches, bench_lookup);
criterion_main!(benches);
