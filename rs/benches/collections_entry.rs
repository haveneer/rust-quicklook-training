//! Counting occurrences: the Entry API against "search, then insert or get_mut".
//! WORDS words drawn from a vocabulary of V distinct keys (the sweep), counted in a
//! fresh map at each iteration.
//!
//! cargo bench --bench collections_entry
//! venv/bin/python benches/plot_figures.py --only collections
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::{BTreeMap, HashMap};

const WORDS: usize = 10_000;
const VOCABULARY: [usize; 4] = [16, 256, 4096, 65536];

/// Deterministic pseudo-random draws in 0..v (no rand: reproducible measurements)
fn draws(v: usize) -> Vec<u64> {
    (0..WORDS as u64)
        .map(|i| (i.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 20) % v as u64)
        .collect()
}

macro_rules! count_ints {
    ($map:ty, $words:expr) => {{
        let words: &[u64] = $words;
        (
            move || {
                let mut m = <$map>::new();
                for &w in words {
                    if m.contains_key(&w) {
                        *m.get_mut(&w).unwrap() += 1;
                    } else {
                        m.insert(w, 1u32);
                    }
                }
                m.len()
            },
            move || {
                let mut m = <$map>::new();
                for &w in words {
                    if let Some(n) = m.get_mut(&w) {
                        *n += 1;
                    } else {
                        m.insert(w, 1u32);
                    }
                }
                m.len()
            },
            move || {
                let mut m = <$map>::new();
                for &w in words {
                    *m.entry(w).or_insert(0u32) += 1;
                }
                m.len()
            },
        )
    }};
}

fn bench_ints(c: &mut Criterion) {
    let mut group = c.benchmark_group("entry_u64");
    for v in VOCABULARY {
        let words = draws(v);
        let (h_contains, h_get_mut, h_entry) = count_ints!(HashMap<u64, u32>, &words);
        let (b_contains, b_get_mut, b_entry) = count_ints!(BTreeMap<u64, u32>, &words);
        let cases: [(&str, &dyn Fn() -> usize); 6] = [
            ("hashmap_contains_then", &h_contains),
            ("hashmap_get_mut_else_insert", &h_get_mut),
            ("hashmap_entry", &h_entry),
            ("btreemap_contains_then", &b_contains),
            ("btreemap_get_mut_else_insert", &b_get_mut),
            ("btreemap_entry", &b_entry),
        ];
        for (name, f) in cases {
            group.bench_function(BenchmarkId::new(name, v), |b| b.iter(|| black_box(f())));
        }
    }
    group.finish();
}

type Counter = fn(&[&str]) -> usize;

// Each hot loop in its own non-inlined function: its machine code no longer depends on
// how criterion's generic code around it gets inlined (see the layout note below).

#[inline(never)]
fn count_get_mut(text: &[&str]) -> usize {
    let mut m: HashMap<String, u32> = HashMap::new();
    for &w in text {
        if let Some(n) = m.get_mut(w) {
            *n += 1; // lookup by &str: no allocation
        } else {
            m.insert(w.to_owned(), 1); // allocates only for a new key
        }
    }
    m.len()
}

#[inline(never)]
fn count_entry_to_owned(text: &[&str]) -> usize {
    let mut m: HashMap<String, u32> = HashMap::new();
    for &w in text {
        *m.entry(w.to_owned()).or_insert(0) += 1; // allocates every time
    }
    m.len()
}

#[inline(never)]
fn count_entry_ref(text: &[&str]) -> usize {
    // std's SipHash, so that only the API differs from the two functions above
    let mut m: hashbrown::HashMap<String, u32, std::hash::RandomState> =
        hashbrown::HashMap::default();
    for &w in text {
        *m.entry_ref(w).or_insert(0) += 1; // allocates only for a new key
    }
    m.len()
}

fn bench_strings(c: &mut Criterion) {
    let mut group = c.benchmark_group("entry_str");
    for v in VOCABULARY {
        let vocabulary: Vec<String> = (0..v).map(|i| format!("word-{i:05}")).collect();
        let text: Vec<&str> = draws(v)
            .iter()
            .map(|&i| vocabulary[i as usize].as_str())
            .collect();
        let cases: [(&str, Counter); 3] = [
            ("get_mut_else_insert", count_get_mut),
            ("entry_to_owned", count_entry_to_owned),
            ("hashbrown_entry_ref", count_entry_ref),
        ];
        for (name, count) in cases {
            group.bench_function(BenchmarkId::new(name, v), |b| {
                b.iter(|| count(black_box(&text)))
            });
        }
    }
    group.finish();
}

criterion_group!(benches, bench_ints, bench_strings);
criterion_main!(benches);
