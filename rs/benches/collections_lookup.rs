//! Membership lookups depending on the collection size: linear scan of a Vec,
//! binary search in a sorted Vec, HashMap (std SipHash, FxHash), hashbrown
//! (foldhash), IndexMap and BTreeMap. Then a full iteration over the same tables.
//!
//! cargo bench --bench collections_lookup
//! venv/bin/python benches/plot_figures.py --only collections
use criterion::measurement::WallTime;
use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion,
};
use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use std::collections::{BTreeMap, HashMap};

const QUERIES: usize = 1000;
const SIZES: [usize; 5] = [8, 64, 512, 4096, 32768];

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
    for n in SIZES {
        let keys: Vec<u64> = (0..n).map(key).collect();
        // Half of the queries hit, half miss
        let queries: Vec<u64> = (0..QUERIES)
            .map(|i| key(if i % 2 == 0 { i % n } else { n + i }))
            .collect();
        let mut sorted = keys.clone();
        sorted.sort_unstable();
        let hash: HashMap<u64, ()> = keys.iter().map(|&k| (k, ())).collect();
        let fx: FxHashMap<u64, ()> = keys.iter().map(|&k| (k, ())).collect();
        let brown: hashbrown::HashMap<u64, ()> = keys.iter().map(|&k| (k, ())).collect();
        let index: IndexMap<u64, ()> = keys.iter().map(|&k| (k, ())).collect();
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
        run(&mut group, "hashmap_fx", n, &queries, |q| {
            fx.contains_key(&q)
        });
        run(&mut group, "hashbrown", n, &queries, |q| {
            brown.contains_key(&q)
        });
        run(&mut group, "indexmap", n, &queries, |q| {
            index.contains_key(&q)
        });
        run(&mut group, "btreemap", n, &queries, |q| {
            btree.contains_key(&q)
        });
    }
    group.finish();
}

/// Sum of all the values: what a full traversal costs, per element
fn bench_iterate(c: &mut Criterion) {
    let mut group = c.benchmark_group("collections_iterate");
    for n in SIZES {
        let pairs: Vec<(u64, u64)> = (0..n).map(|i| (key(i), i as u64)).collect();
        let vec = pairs.clone();
        let hash: HashMap<u64, u64> = pairs.iter().copied().collect();
        let index: IndexMap<u64, u64> = pairs.iter().copied().collect();
        let btree: BTreeMap<u64, u64> = pairs.iter().copied().collect();

        let id = |name| BenchmarkId::new(name, n);
        group.bench_function(id("vec"), |b| {
            b.iter(|| black_box(&vec).iter().map(|(_, v)| v).sum::<u64>())
        });
        group.bench_function(id("hashmap"), |b| {
            b.iter(|| black_box(&hash).values().sum::<u64>())
        });
        group.bench_function(id("indexmap"), |b| {
            b.iter(|| black_box(&index).values().sum::<u64>())
        });
        group.bench_function(id("btreemap"), |b| {
            b.iter(|| black_box(&btree).values().sum::<u64>())
        });
    }
    group.finish();
}

criterion_group!(benches, bench_lookup, bench_iterate);
criterion_main!(benches);
