// cargo run --example debug_channel_hang   (never ends: attach a debugger)
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    for id in 0..4 {
        let tx = tx.clone(); // one Sender per worker
        thread::spawn(move || tx.send(id * id).unwrap());
    }
    // drop(tx); // FIX: main's own Sender keeps the channel open
    let total: i32 = rx.iter().sum(); // until no Sender is left
    println!("total = {total}");
}
