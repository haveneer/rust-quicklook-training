use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0); // the data lives inside the lock
    thread::scope(|s| {
        for _ in 0..2 {
            s.spawn(|| {
                for _ in 0..1_000_000 {
                    let mut guard = counter.lock().unwrap(); // only way to reach the data
                    *guard += 1;
                } // guard dropped here: lock released
            });
        }
    });
    let total = counter.into_inner().unwrap();
    println!("counter = {total}");
    assert_eq!(total, 2_000_000);
}

#[test]
fn test() {
    main()
}
