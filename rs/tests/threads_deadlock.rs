use std::sync::Mutex;
use std::thread;
use std::time::Duration;

// Crossed lock order: thread 1 holds 0 and waits for 1, thread 2 holds 1 and waits for 0
#[allow(dead_code)]
fn transfer_deadlock(cells: &[Mutex<f64>], from: usize, to: usize, amount: f64) {
    let mut src = cells[from].lock().unwrap();
    thread::sleep(Duration::from_millis(10)); // widens the window
    let mut dst = cells[to].lock().unwrap(); // may wait forever
    *src -= amount;
    *dst += amount;
}

// Same global order for everyone: lowest index first
fn transfer(cells: &[Mutex<f64>], from: usize, to: usize, amount: f64) {
    let (first, second) = (from.min(to), from.max(to));
    let mut g1 = cells[first].lock().unwrap();
    let mut g2 = cells[second].lock().unwrap();
    let (src, dst) = if from == first {
        (&mut *g1, &mut *g2)
    } else {
        (&mut *g2, &mut *g1)
    };
    *src -= amount;
    *dst += amount;
}

fn main() {
    let cells = [Mutex::new(100.0), Mutex::new(100.0)];
    thread::scope(|s| {
        s.spawn(|| (0..100).for_each(|_| transfer(&cells, 0, 1, 1.0)));
        s.spawn(|| (0..100).for_each(|_| transfer(&cells, 1, 0, 1.0)));
    }); // TODO with transfer_deadlock instead: the program hangs
    let total: f64 = cells.iter().map(|c| *c.lock().unwrap()).sum();
    println!("total = {total}");
    assert_eq!(total, 200.0);
}

#[test]
fn test() {
    main()
}
