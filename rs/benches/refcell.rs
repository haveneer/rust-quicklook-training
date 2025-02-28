use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::cell::RefCell;
use std::sync::Mutex;

fn bench_vec(c: &mut Criterion) {
    c.bench_function("push/pop-on Vec ", |b| {
        let mut vec = Vec::with_capacity(1000);
        b.iter(|| {
            vec.push(black_box(1));
            vec.pop();
        })
    });
}

fn bench_refcell_vec(c: &mut Criterion) {
    c.bench_function("push/pop-RefCell on Vec", |b| {
        let vec = RefCell::new(Vec::with_capacity(1000));
        b.iter(|| {
            let mut ref_mut = vec.borrow_mut();
            ref_mut.push(black_box(1));
            ref_mut.pop();
        })
    });
}

fn bench_mutex_vec(c: &mut Criterion) {
    c.bench_function("push/pop-Mutex on Vec", |b| {
        let vec = Mutex::new(Vec::with_capacity(1000));
        b.iter(|| {
            let mut ref_mut = vec.lock().unwrap();
            ref_mut.push(black_box(1));
            ref_mut.pop();
        })
    });
}

// Regroupe les benchmarks et définit le point d’entrée Criterion
criterion_group!(benches, bench_vec, bench_refcell_vec, bench_mutex_vec);
criterion_main!(benches);
