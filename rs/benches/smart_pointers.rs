use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use std::cell::RefCell;
use std::ops::Deref;
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_creation(c: &mut Criterion) {
    // Benchmark heap allocation for each pointer
    c.bench_function("create-box", |b| b.iter(|| {
        // Box allocates on the heap
        black_box(Box::new(black_box(42)));
    }));
    c.bench_function("create-rc", |b| b.iter(|| {
        black_box(Rc::new(black_box(42)));
    }));
    c.bench_function("create-arc", |b| b.iter(|| {
        black_box(Arc::new(black_box(42)));
    }));
    c.bench_function("create-refcell", |b| b.iter(|| {
        // RefCell does not heap-allocate the inner value
        black_box(RefCell::new(black_box(42)));
    }));
    c.bench_function("create-arc_mutex", |b| b.iter(|| {
        // Arc<Mutex<T>> allocates the Mutex and T on the heap
        black_box(Arc::new(Mutex::new(black_box(42))));
    }));
    c.bench_function("create-arc_rwlock", |b| b.iter(|| {
        // Arc<RwLock<T>> allocates the RwLock and T on the heap
        black_box(Arc::new(RwLock::new(black_box(42))));
    }));
}

fn bench_clone(c: &mut Criterion) {
    // Prepare one instance of each pointer outside the loop
    let boxed = Box::new(42);
    let rc = Rc::new(42);
    let arc = Arc::new(42);
    let cell = RefCell::new(42);
    let arc_mutex = Arc::new(Mutex::new(42));
    let arc_rwlock = Arc::new(RwLock::new(42));

    c.bench_function("clone-box", |b| b.iter(|| {
        // Box<T>::clone performs a deep copy (allocates new box)
        let new_box = boxed.clone();              // requires T: Clone
        black_box(new_box);
    }));
    c.bench_function("clone-rc", |b| b.iter(|| {
        let rc2 = rc.clone();                // increments ref count
        black_box(&rc2);
    }));
    c.bench_function("clone-arc", |b| b.iter(|| {
        let arc2 = arc.clone();              // increments atomic ref count
        black_box(&arc2);
    }));
    c.bench_function("clone-refcell", |b| b.iter(|| {
        let cell2 = cell.clone();            // clones inner value (deep copy)
        black_box(cell2);
    }));
    c.bench_function("clone-arc_mutex", |b| b.iter(|| {
        let arc2 = arc_mutex.clone();        // increments atomic ref count, shallow clone
        black_box(&arc2);
    }));
    c.bench_function("clone-arc_rwlock", |b| b.iter(|| {
        let arc2 = arc_rwlock.clone();        // increments atomic ref count, shallow clone
        black_box(&arc2);
    }));
}

fn bench_access(c: &mut Criterion) {
    // Prepare one instance of each pointer
    let boxed = Box::new(42);
    let rc = Rc::new(42);
    let arc = Arc::new(42);
    let cell = RefCell::new(42);
    let arc_mutex = Arc::new(Mutex::new(42));
    let arc_rwlock = Arc::new(RwLock::new(42));

    c.bench_function("access-box", |b| b.iter(|| {
        // Dereference Box
        let val = boxed.deref();
        black_box(val);
    }));
    c.bench_function("access-rc", |b| b.iter(|| {
        // Dereference Rc (similar to Box deref)
        let val = *rc;
        black_box(val);
    }));
    c.bench_function("access-arc", |b| b.iter(|| {
        // Dereference Arc (similar to Rc deref)
        let val = *arc;
        black_box(val);
    }));
    c.bench_function("access-refcell", |b| b.iter(|| {
        // Borrow from RefCell (runtime borrow check) then deref
        let borrow = cell.borrow();
        let val = *borrow;         // `borrow` is a Ref<T>
        black_box(val);
        // `borrow` drops here, releasing the RefCell borrow
    }));
    c.bench_function("access-mutex", |b| b.iter(|| {
        // Lock the Mutex (acquire lock) then deref
        let guard = arc_mutex.lock().unwrap();
        let val = *guard;
        black_box(val);
        // guard drops here, releasing the lock
    }));
    c.bench_function("access-rwlock_read", |b| b.iter(|| {
        // Read access to the RwLock (acquire lock) then deref
        let guard = arc_rwlock.read().unwrap();
        let val = *guard;
        black_box(val);
        // guard drops here, releasing the lock
    }));
    c.bench_function("access-rwlock_write", |b| b.iter(|| {
        // Write access to the RwLock (acquire lock) then deref
        let guard = arc_rwlock.write().unwrap();
        let val = *guard;
        black_box(val);
        // guard drops here, releasing the lock
    }));
}

// Group benchmarks and define main entry
criterion_group!(benches, bench_creation, bench_clone, bench_access);
criterion_main!(benches);
