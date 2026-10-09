//! Hash functions behind the same std HashMap: 1000 lookups (50 % hits) in a table
//! of 4096 keys, for u64 keys, short strings (8 bytes) and long strings (256 bytes).
//!
//! cargo bench --bench collections_hashers
//! venv/bin/python benches/plot_figures.py --only collections
use criterion::measurement::WallTime;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkGroup, Criterion};
use std::collections::HashMap;
use std::hash::{BuildHasher, BuildHasherDefault, Hash, RandomState};

const KEYS: usize = 4096;
const QUERIES: usize = 1000;

fn mix(i: usize) -> u64 {
    (i as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 16
}

fn lookups<K: Hash + Eq, S: BuildHasher + Default>(
    group: &mut BenchmarkGroup<WallTime>,
    name: &str,
    keys: &[K],
    queries: &[K],
) {
    let map: HashMap<&K, (), S> = keys.iter().map(|k| (k, ())).collect();
    group.bench_function(name, |b| {
        b.iter(|| {
            queries
                .iter()
                .filter(|q| map.contains_key(black_box(q)))
                .count()
        })
    });
}

/// One line per hash function, same keys and queries for all
fn all_hashers<K: Hash + Eq>(c: &mut Criterion, group: &str, key: impl Fn(usize) -> K) {
    let keys: Vec<K> = (0..KEYS).map(&key).collect();
    let queries: Vec<K> = (0..QUERIES)
        .map(|i| key(if i % 2 == 0 { i % KEYS } else { KEYS + i }))
        .collect();
    let mut g = c.benchmark_group(group);
    lookups::<K, RandomState>(&mut g, "siphash", &keys, &queries);
    lookups::<K, foldhash::fast::RandomState>(&mut g, "foldhash", &keys, &queries);
    lookups::<K, rustc_hash::FxBuildHasher>(&mut g, "fxhash", &keys, &queries);
    lookups::<K, fnv::FnvBuildHasher>(&mut g, "fnv", &keys, &queries);
    lookups::<K, ahash::RandomState>(&mut g, "ahash", &keys, &queries);
    type Xxh3 = BuildHasherDefault<twox_hash::XxHash3_64>;
    lookups::<K, Xxh3>(&mut g, "xxh3", &keys, &queries);
    lookups::<K, rapidhash::fast::RandomState>(&mut g, "rapidhash", &keys, &queries);
    g.finish();
}

fn bench_hashers(c: &mut Criterion) {
    all_hashers(c, "hashers_u64", mix);
    // nohash only makes sense for integer keys: hash(k) = k
    {
        let keys: Vec<u64> = (0..KEYS).map(mix).collect();
        let queries: Vec<u64> = (0..QUERIES)
            .map(|i| mix(if i % 2 == 0 { i % KEYS } else { KEYS + i }))
            .collect();
        let map: nohash_hasher::IntMap<u64, ()> = keys.iter().map(|&k| (k, ())).collect();
        let mut g = c.benchmark_group("hashers_u64");
        g.bench_function("nohash", |b| {
            b.iter(|| {
                queries
                    .iter()
                    .filter(|q| map.contains_key(black_box(q)))
                    .count()
            })
        });
        g.finish();
    }
    all_hashers(c, "hashers_short", |i| format!("{:08x}", mix(i) as u32));
    all_hashers(c, "hashers_long", |i| format!("{:0256x}", mix(i)));
}

criterion_group!(benches, bench_hashers);
criterion_main!(benches);
