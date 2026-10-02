use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};
use std::thread;

/// 1) Lecture concurrente
///
/// - Compare la lecture via Arc<Vec<usize>> (pas de lock) et Arc<Mutex<Vec<usize>>> (avec lock).
/// - On veut voir le surcoût d'un lock pour une opération de lecture seule.
///
fn bench_concurrent_read_arc_vs_arc_mutex(c: &mut Criterion) {
    let mut group = c.benchmark_group("scenario-1-concurrent-read");

    // Paramètres
    const ARRAY_SIZE: usize = 1_000_000;
    const THREADS: usize = 4;

    // Préparation des données : un grand vecteur
    let data = (0..ARRAY_SIZE).collect::<Vec<usize>>();

    // A) Arc<Vec<usize>> - Pas de lock en lecture
    group.bench_function("concurrent_read_arc_vec", |b| {
        // On met data dans un Arc
        let arc_vec = Arc::new(data.clone());
        b.iter(|| {
            let mut handles = Vec::with_capacity(THREADS);
            for _ in 0..THREADS {
                let cloned = Arc::clone(&arc_vec);
                handles.push(thread::spawn(move || {
                    // Opération de lecture : somme de tous les éléments
                    let sum: usize = cloned.iter().sum();
                    black_box(sum);
                }));
            }
            // Join
            for h in handles {
                h.join().unwrap();
            }
        });
    });

    // B) Arc<Mutex<Vec<usize>>> - Force un lock même en lecture
    group.bench_function("concurrent_read_arc_mutex_vec", |b| {
        // On met data dans un Arc<Mutex<Vec<usize>>>
        let arc_mutex_vec = Arc::new(Mutex::new(data.clone()));
        b.iter(|| {
            let mut handles = Vec::with_capacity(THREADS);
            for _ in 0..THREADS {
                let cloned = Arc::clone(&arc_mutex_vec);
                handles.push(thread::spawn(move || {
                    // Lock pour lire
                    let guard = cloned.lock().unwrap();
                    let sum: usize = guard.iter().sum();
                    black_box(sum);
                }));
            }
            // Join
            for h in handles {
                h.join().unwrap();
            }
        });
    });

    group.finish();
}

/// 2) Ecriture concurrente
///
/// - Chaque thread clone Arc<Mutex<usize>> et incrémente ITERATIONS fois.
/// - Mesure la contention sur un même Mutex.
///
/// Paramètres : THREADS = 4, ITERATIONS = 100k
///
fn bench_concurrent_write_arc_mutex(c: &mut Criterion) {
    let mut group = c.benchmark_group("scenario-2-concurrent-write");

    // Paramètres
    const THREADS: usize = 4;
    const ITERATIONS: usize = 100_000;

    group.bench_function("concurrent_write_arc_mutex", |b| {
        b.iter(|| {
            // Arc<Mutex> initialisé à 0
            let shared = Arc::new(Mutex::new(0_usize));
            let mut handles = Vec::with_capacity(THREADS);

            for _ in 0..THREADS {
                let cloned = Arc::clone(&shared);
                handles.push(thread::spawn(move || {
                    for _ in 0..ITERATIONS {
                        let mut guard = cloned.lock().unwrap();
                        *guard += 1;
                    }
                }));
            }

            for h in handles {
                h.join().unwrap();
            }

            // Valeur finale (black_box pour éviter optimisation)
            let final_val = *shared.lock().unwrap();
            black_box(final_val);
        });
    });

    group.finish();
}

/// 3) Approche lock-free
///
/// Compare la performance de la même opération (incrémentation) entre:
///  - Arc<AtomicUsize> (pas de lock)
///  - Arc<Mutex<usize>> (avec lock)
///
fn bench_lockfree_vs_mutex(c: &mut Criterion) {
    let mut group = c.benchmark_group("scenario-3-lockfree-vs-mutex");

    const THREADS: usize = 4;
    const ITERATIONS: usize = 100_000;

    // A) Arc<AtomicUsize>
    group.bench_function("arc_atomic_increment", |b| {
        b.iter(|| {
            let atomic_val = Arc::new(AtomicUsize::new(0));
            let mut handles = Vec::with_capacity(THREADS);

            for _ in 0..THREADS {
                let cloned = Arc::clone(&atomic_val);
                handles.push(thread::spawn(move || {
                    for _ in 0..ITERATIONS {
                        cloned.fetch_add(1, Ordering::SeqCst);
                    }
                }));
            }

            for h in handles {
                h.join().unwrap();
            }

            black_box(atomic_val.load(Ordering::SeqCst));
        });
    });

    // B) Arc<Mutex<usize>>
    group.bench_function("arc_mutex_increment", |b| {
        b.iter(|| {
            let mutex_val = Arc::new(Mutex::new(0_usize));
            let mut handles = Vec::with_capacity(THREADS);

            for _ in 0..THREADS {
                let cloned = Arc::clone(&mutex_val);
                handles.push(thread::spawn(move || {
                    for _ in 0..ITERATIONS {
                        let mut guard = cloned.lock().unwrap();
                        *guard += 1;
                    }
                }));
            }

            for h in handles {
                h.join().unwrap();
            }

            let final_val = *mutex_val.lock().unwrap();
            black_box(final_val);
        });
    });

    group.finish();
}

/// 4) Petit "bonus" de comparaison naive RefCell (monothread) vs Arc<Mutex> (multithread)
///
/// Evidemment, `RefCell` ne s'enverra pas entre threads, mais on peut faire un bench monothread
/// juste pour voir la diff.
fn bench_refcell_vs_mutex(c: &mut Criterion) {
    use std::cell::RefCell;

    let mut group = c.benchmark_group("scenario-4-refcell-vs-mutex");

    const ITERATIONS: usize = 1_000_000;

    // A) RefCell monothread
    group.bench_function("refcell_increment_monothread", |b| {
        b.iter(|| {
            let cell = RefCell::new(0_usize);
            for _ in 0..ITERATIONS {
                *cell.borrow_mut() += 1;
            }
            black_box(cell.into_inner());
        });
    });

    // B) Arc<Mutex> monothread (pour comparer le surcoût d'un lock, même monothread)
    group.bench_function("mutex_increment_monothread", |b| {
        b.iter(|| {
            let shared = Arc::new(Mutex::new(0_usize));
            // Monothread, donc 1 thread
            for _ in 0..ITERATIONS {
                let mut guard = shared.lock().unwrap();
                *guard += 1;
            }
            black_box(*shared.lock().unwrap());
        });
    });

    group.finish();
}

criterion_group!(
    concurrent_real_scenarios,
    bench_concurrent_read_arc_vs_arc_mutex,
    bench_concurrent_write_arc_mutex,
    bench_lockfree_vs_mutex,
    bench_refcell_vs_mutex
);
criterion_main!(concurrent_real_scenarios);
