use std::sync::{RwLock, TryLockError};
use std::thread;
use std::time::Duration;

fn main() {
    let mesh = RwLock::new(vec![0.0_f64; 4]);

    thread::scope(|s| {
        let first = mesh.read().unwrap(); // this thread holds a read guard

        s.spawn(|| {
            *mesh.write().unwrap() = vec![1.0; 4]; // a writer starts waiting
            println!("writer done");
        });
        thread::sleep(Duration::from_millis(50)); // let the writer queue up

        // A second read() here would block behind the waiting writer,
        // which itself waits for `first`: deadlock. try_read shows it.
        match mesh.try_read() {
            Err(TryLockError::WouldBlock) => println!("2nd read() would deadlock"),
            Ok(_) => println!("2nd read() granted (reader-preferring lock)"),
            Err(TryLockError::Poisoned(e)) => panic!("{e}"),
        }
        println!("first sees {:?}", *first);
        drop(first); // releasing it lets the writer proceed
    });
}

#[test]
fn test() {
    main()
}
