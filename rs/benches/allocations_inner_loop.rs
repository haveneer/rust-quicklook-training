use criterion::{black_box, criterion_group, criterion_main, Criterion};

const N: usize = 1000; // Number of small arrays
const M: usize = 1000; // Size of each array

/// Benchmark 1: Allocation time for multiple small arrays.
fn bench_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("allocations");

    // Stack allocation: Allocate P arrays of size M on the stack (each inside the loop)
    group.bench_function("Stack allocation (array)", |b| {
        b.iter(|| {
            for _ in 0..N {
                let arr = [0i32; M];
                black_box(&arr);
            }
        })
    });

    // Heap allocation: Allocate P Vecs of size M on the heap.
    group.bench_function("Heap allocation (Vec)", |b| {
        b.iter(|| {
            for _ in 0..N {
                let vec = vec![0i32; M];
                black_box(&vec);
            }
        })
    });

    group.finish();
}

/// Benchmark 2: Sequential access (read/write) on multiple small arrays.
fn bench_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("accesses");

    // Stack allocation sequential access: Allocate and update each array in a loop.
    group.bench_function("Sequential access on stack (array)", |b| {
        let mut arr = [0i32; M];
        b.iter(|| {
            for _ in 0..N {
                for i in 0..M {
                    arr[i] += 1;
                }
                black_box(&arr);
            }
        })
    });

    // Heap allocation sequential access: Allocate a Vec and update its elements.
    group.bench_function("Sequential access on heap (Vec)", |b| {
        let mut vec = vec![0i32; M];
        b.iter(|| {
            for _ in 0..N {
                for i in 0..vec.len() {
                    vec[i] += 1;
                }
                black_box(&vec);
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bench_allocation, bench_access);
criterion_main!(benches);
