// cargo build --example debug_deadlock
// rust-lldb target/debug/examples/debug_deadlock   then: run, Ctrl-C, thread backtrace all
use std::{sync::Mutex, thread, time::Duration};

// Crossed lock order: thread 1 holds 0 and waits for 1, thread 2 holds 1 and waits for 0
fn transfer(cells: &[Mutex<f64>], from: usize, to: usize, amount: f64) {
    let mut src = cells[from].lock().unwrap();
    thread::sleep(Duration::from_millis(10)); // widens the window
    let mut dst = cells[to].lock().unwrap(); // waits forever
    *src -= amount;
    *dst += amount;
}

fn main() {
    let cells = [Mutex::new(100.0), Mutex::new(100.0)];
    thread::scope(|s| {
        s.spawn(|| transfer(&cells, 0, 1, 1.0));
        s.spawn(|| transfer(&cells, 1, 0, 1.0));
    }); // never returns
}
