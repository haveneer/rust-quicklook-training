//! std::sync vs parking_lot: same workload, only the lock type changes.
//! Every case performs OPS lock acquisitions in total, split across the threads,
//! each guarding a trivial critical section (worst case: contention is maximal).
//!
//! cargo bench --bench concurrent_std_vs_parking_lot
//! venv/bin/python benches/plot_figures.py --only concurrency
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::thread;

const OPS: usize = 400_000;

/// The minimal API both implementations share
trait Lock: Sync {
    fn new(v: u64) -> Self;
    fn incr(&self);
}

trait RwLockLike: Lock {
    fn get(&self) -> u64;
}

impl Lock for std::sync::Mutex<u64> {
    fn new(v: u64) -> Self {
        std::sync::Mutex::new(v)
    }
    fn incr(&self) {
        *self.lock().unwrap() += 1; // poisoning: lock() returns a Result
    }
}

impl Lock for parking_lot::Mutex<u64> {
    fn new(v: u64) -> Self {
        parking_lot::Mutex::new(v)
    }
    fn incr(&self) {
        *self.lock() += 1; // no poisoning: lock() returns the guard
    }
}

impl Lock for std::sync::RwLock<u64> {
    fn new(v: u64) -> Self {
        std::sync::RwLock::new(v)
    }
    fn incr(&self) {
        *self.write().unwrap() += 1;
    }
}

impl RwLockLike for std::sync::RwLock<u64> {
    fn get(&self) -> u64 {
        *self.read().unwrap()
    }
}

impl Lock for parking_lot::RwLock<u64> {
    fn new(v: u64) -> Self {
        parking_lot::RwLock::new(v)
    }
    fn incr(&self) {
        *self.write() += 1;
    }
}

impl RwLockLike for parking_lot::RwLock<u64> {
    fn get(&self) -> u64 {
        *self.read()
    }
}

/// `threads` threads share OPS increments of the same counter
fn increments<L: Lock>(threads: usize) -> L {
    let lock = L::new(0);
    thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| {
                for _ in 0..OPS / threads {
                    black_box(&lock).incr();
                }
            });
        }
    });
    lock
}

/// 4 threads share OPS accesses, one write every `write_every` accesses
fn reads<L: RwLockLike>(write_every: usize) -> u64 {
    const THREADS: usize = 4;
    let lock = L::new(0);
    thread::scope(|s| {
        for _ in 0..THREADS {
            s.spawn(|| {
                let mut sum = 0;
                for i in 0..OPS / THREADS {
                    if write_every != 0 && i % write_every == 0 {
                        black_box(&lock).incr();
                    } else {
                        sum += black_box(&lock).get();
                    }
                }
                black_box(sum);
            });
        }
    });
    lock.get()
}

fn bench_mutex(c: &mut Criterion) {
    for threads in [1, 4, 8] {
        let mut g = c.benchmark_group(format!("lock_mutex_{threads}t"));
        g.bench_function("std", |b| {
            b.iter(|| increments::<std::sync::Mutex<u64>>(threads))
        });
        g.bench_function("parking_lot", |b| {
            b.iter(|| increments::<parking_lot::Mutex<u64>>(threads))
        });
        g.finish();
    }
}

fn bench_rwlock(c: &mut Criterion) {
    for (name, write_every) in [("read_only", 0), ("read_mostly", 10)] {
        let mut g = c.benchmark_group(format!("lock_rwlock_{name}_4t"));
        g.bench_function("std", |b| {
            b.iter(|| reads::<std::sync::RwLock<u64>>(write_every))
        });
        g.bench_function("parking_lot", |b| {
            b.iter(|| reads::<parking_lot::RwLock<u64>>(write_every))
        });
        g.finish();
    }
}

criterion_group!(benches, bench_mutex, bench_rwlock);
criterion_main!(benches);
