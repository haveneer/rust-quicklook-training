use std::sync::RwLock;
use std::thread;
use std::time::Duration;

fn main() {
    let cfl = RwLock::new(0.5_f64); // a setting read often, written rarely

    thread::scope(|s| {
        for id in 0..3 {
            let cfl = &cfl;
            s.spawn(move || {
                let value = cfl.read().unwrap(); // many readers at the same time
                println!("reader {id} sees cfl = {value}");
                thread::sleep(Duration::from_millis(20)); // readers overlap here
            });
        }
        s.spawn(|| {
            thread::sleep(Duration::from_millis(5));
            *cfl.write().unwrap() = 0.8; // waits until no reader is left
            println!("writer updated cfl");
        });
    });

    // A writer is exclusive: try_write fails while a read guard is alive
    let guard = cfl.read().unwrap();
    assert!(cfl.try_write().is_err());
    drop(guard);
    println!("final cfl = {}", cfl.read().unwrap());
}

#[test]
fn test() {
    main()
}
