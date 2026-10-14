//! Sequences: FIFO queue and priority queue on Vec, VecDeque,
//! LinkedList, BinaryHeap and BTreeSet, depending on the number of elements n.
//!
//! cargo bench --bench collections_sequences
//! venv/bin/python benches/plot_figures.py --only collections
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::collections::{BTreeSet, BinaryHeap, LinkedList, VecDeque};
use std::hint::black_box;

const SIZES: [usize; 7] = [8, 32, 128, 512, 2048, 8192, 32768];

/// Deterministic pseudo-random priorities
fn prio(i: usize) -> u64 {
    (i as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 32
}

/// Steady-state queue of n elements: push one at the back, pop one at the front, n times
fn bench_fifo(c: &mut Criterion) {
    let mut group = c.benchmark_group("seq_fifo");
    for n in SIZES {
        let id = |name| BenchmarkId::new(name, n);
        let mut vec: Vec<u64> = (0..n as u64).collect();
        group.bench_function(id("vec_remove0"), |b| {
            b.iter(|| {
                for i in 0..n as u64 {
                    vec.push(i);
                    black_box(vec.remove(0)); // shifts the whole Vec
                }
            })
        });
        let mut deque: VecDeque<u64> = (0..n as u64).collect();
        group.bench_function(id("vecdeque"), |b| {
            b.iter(|| {
                for i in 0..n as u64 {
                    deque.push_back(i);
                    black_box(deque.pop_front());
                }
            })
        });
        let mut list: LinkedList<u64> = (0..n as u64).collect();
        group.bench_function(id("linkedlist"), |b| {
            b.iter(|| {
                for i in 0..n as u64 {
                    list.push_back(i); // one allocation...
                    black_box(list.pop_front()); // ...and one free per element
                }
            })
        });
    }
    group.finish();
}

/// Priority queue: push n priorities, then pop them all in order
fn bench_priority(c: &mut Criterion) {
    let mut group = c.benchmark_group("seq_priority");
    for n in SIZES {
        let id = |name| BenchmarkId::new(name, n);
        let input: Vec<u64> = (0..n).map(prio).collect();
        group.bench_function(id("binaryheap"), |b| {
            b.iter(|| {
                let mut heap = BinaryHeap::with_capacity(n);
                for &p in black_box(&input) {
                    heap.push(p);
                }
                let mut last = 0;
                while let Some(p) = heap.pop() {
                    last = p;
                }
                last
            })
        });
        group.bench_function(id("btreeset"), |b| {
            b.iter(|| {
                // (priority, rank): a set would merge equal priorities otherwise
                let mut set = BTreeSet::new();
                for (rank, &p) in black_box(&input).iter().enumerate() {
                    set.insert((p, rank));
                }
                let mut last = 0;
                while let Some((p, _)) = set.pop_last() {
                    last = p;
                }
                last
            })
        });
        group.bench_function(id("sorted_vec_insert"), |b| {
            b.iter(|| {
                let mut sorted: Vec<u64> = Vec::with_capacity(n);
                for &p in black_box(&input) {
                    let at = sorted.partition_point(|&x| x < p);
                    sorted.insert(at, p); // keeps the Vec sorted: O(n) shift
                }
                let mut last = 0;
                while let Some(p) = sorted.pop() {
                    last = p;
                }
                last
            })
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fifo, bench_priority);
criterion_main!(benches);
