use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

fn main() {
    // A counter shared by 4 threads, without any lock
    let evaluated = AtomicUsize::new(0);
    let stop = AtomicBool::new(false);

    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                for _ in 0..10_000 {
                    // read-modify-write as one indivisible operation
                    evaluated.fetch_add(1, Ordering::Relaxed); // only the count matters
                }
            });
        }
        s.spawn(|| {
            while !stop.load(Ordering::Acquire) {
                thread::sleep(Duration::from_millis(1)); // e.g. a progress monitor
            }
            println!("monitor stopped");
        });
        thread::sleep(Duration::from_millis(20));
        stop.store(true, Ordering::Release); // pairs with the Acquire load above
    });

    // No lost update, unlike a plain `+= 1` on a shared integer (which does not compile)
    assert_eq!(evaluated.load(Ordering::Relaxed), 40_000);
    println!("evaluated = {}", evaluated.into_inner());
}

#[test]
fn test() {
    main()
}
