use bumpalo::Bump;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::seq::SliceRandom;
use rand::thread_rng;

const N: usize = 1_000_000; // Total number of objects
const M: usize = 2; // Number of fields

struct SmallObject {
    data: [u32; M],
}

impl SmallObject {
    fn new(i: usize) -> SmallObject {
        Self {
            data: [i as u32; M],
        }
    }

    fn update(&mut self) {
        for i in 0..M {
            self.data[i] += 1;
        }
    }
}

/// Benchmark allocation of many small objects using Box.
fn bench_box_allocation(c: &mut Criterion) {
    c.bench_function("Box allocation for many small objects", |b| {
        b.iter(|| {
            let mut objects = Vec::with_capacity(N);
            for i in 0..N {
                objects.push(Box::new(SmallObject::new(i)));
            }
            black_box(objects);
        })
    });
}

/// Benchmark allocation of many small objects using an arena.
fn bench_arena_allocation(c: &mut Criterion) {
    c.bench_function("Arena allocation for many small objects", |b| {
        b.iter(|| {
            let arena = Bump::with_capacity(N * size_of::<SmallObject>());
            let mut objects = Vec::with_capacity(N);
            for i in 0..N {
                let obj = arena.alloc(SmallObject::new(i));
                objects.push(obj);
            }
            black_box(objects);
        })
    });
}

/// Benchmark sequential access (read/write) on the allocated objects.
/// This simulates a workload where after allocation, each object is updated.
fn bench_object_access(c: &mut Criterion) {
    c.bench_function("Sequential access on Box-allocated objects", |b| {
        let mut objects = Vec::with_capacity(N);
        for i in 0..N {
            objects.push(Box::new(SmallObject::new(i)));
        }
        b.iter(|| {
            for obj in objects.iter_mut() {
                obj.update();
            }
            black_box(&objects);
        })
    });

    c.bench_function("Sequential access on arena-allocated objects", |b| {
        let arena = Bump::with_capacity(N * size_of::<SmallObject>());
        let mut objects = Vec::with_capacity(N);
        for i in 0..N {
            let obj = arena.alloc(SmallObject::new(i));
            objects.push(obj);
        }
        b.iter(|| {
            for obj in objects.iter_mut() {
                obj.update();
            }
            black_box(&objects);
        })
    });
}

/// Benchmark random access (read/write) on the allocated objects.
/// The same random order is used for both the Box and the arena cases.
fn bench_random_access(c: &mut Criterion) {
    // Generate a random permutation of indices from 0 to N-1.
    let mut indices: Vec<usize> = (0..N).collect();
    indices.shuffle(&mut thread_rng());
    // Protect the random order from being optimized away.
    let indices = black_box(indices);

    // Benchmark for Box-based allocation.
    c.bench_function("Random access on Box-allocated objects", |b| {
        let mut objects = Vec::with_capacity(N);
        for i in 0..N {
            objects.push(Box::new(SmallObject::new(i)));
        }
        b.iter(|| {
            // Update object at random index.
            for &i in &indices {
                objects[i].update();
            }
            black_box(&objects);
        })
    });

    // Benchmark for arena-based allocation.
    c.bench_function("Random access on arena-allocated objects", |b| {
        let arena = Bump::with_capacity(N * size_of::<SmallObject>());
        let mut objects = Vec::with_capacity(N);
        for i in 0..N {
            let obj = arena.alloc(SmallObject::new(i));
            objects.push(obj);
        }
        b.iter(|| {
            for &i in &indices {
                objects[i].update();
            }
            black_box(&objects);
        })
    });
}

criterion_group!(
    benches,
    bench_box_allocation,
    bench_arena_allocation,
    bench_object_access,
    bench_random_access
);
criterion_main!(benches);
