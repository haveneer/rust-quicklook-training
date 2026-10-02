use std::sync::{Barrier, RwLock};
use std::thread;

const N: usize = 4;

fn main() {
    // 1D Jacobi smoothing: each thread owns one cell, reads its neighbours
    let cells: Vec<RwLock<f64>> = [0.0, 0.0, 8.0, 0.0].map(RwLock::new).into();
    let barrier = Barrier::new(N);

    thread::scope(|s| {
        for i in 0..N {
            let (cells, barrier) = (&cells, &barrier);
            s.spawn(move || {
                for step in 0..3 {
                    let left = *cells[(i + N - 1) % N].read().unwrap();
                    let right = *cells[(i + 1) % N].read().unwrap();
                    let new = (left + *cells[i].read().unwrap() + right) / 3.0;
                    barrier.wait(); // everybody has read the old values...
                    *cells[i].write().unwrap() = new;
                    if barrier.wait().is_leader() {
                        // ...and written the new ones; exactly one thread reports
                        let v: Vec<f64> = cells.iter().map(|c| *c.read().unwrap()).collect();
                        println!("step {step}: {v:.3?}");
                    }
                    barrier.wait(); // the leader has printed before the next step reads
                }
            });
        }
    });
    let total: f64 = cells.iter().map(|c| *c.read().unwrap()).sum();
    assert!((total - 8.0).abs() < 1e-12); // smoothing conserves the sum
}

#[test]
fn test() {
    main()
}
