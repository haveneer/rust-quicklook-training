use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();
    let ms = move || start.elapsed().as_millis();

    // Bounded: at most 2 messages in flight, send() blocks when the buffer is full
    let (tx, rx) = mpsc::sync_channel::<u32>(2);
    let producer = thread::spawn(move || {
        for i in 0..5 {
            tx.send(i).unwrap(); // waits for room: back-pressure on the fast producer
            println!("{:>4} ms  sent {i}", ms());
        }
    });
    for value in rx {
        thread::sleep(Duration::from_millis(50)); // slow consumer
        println!("{:>4} ms  processed {value}", ms());
    }
    producer.join().unwrap();

    // Capacity 0: a rendezvous, send() returns only once recv() has taken the value
    let (tx, rx) = mpsc::sync_channel::<&str>(0);
    let h = thread::spawn(move || tx.send("handshake").unwrap());
    thread::sleep(Duration::from_millis(30));
    println!("{:>4} ms  got {}", ms(), rx.recv().unwrap());
    h.join().unwrap();
}

#[test]
fn test() {
    main()
}
