use std::sync::Mutex;
use std::thread;

fn main() {
    let results = Mutex::new(vec![1.0, 2.0]);

    thread::scope(|s| {
        let h = s.spawn(|| {
            let mut v = results.lock().unwrap();
            v.push(f64::NAN); // half-done update...
            panic!("solver diverged"); // ...the guard is dropped during unwinding
        });
        assert!(h.join().is_err());
    });

    // The Mutex is now poisoned: the data may break an invariant
    assert!(results.is_poisoned());
    match results.lock() {
        Ok(_) => unreachable!(),
        Err(poisoned) => {
            // Deliberate choice: take the data anyway, then repair it
            let mut v = poisoned.into_inner();
            v.retain(|x| !x.is_nan());
            println!("recovered {v:?}");
        }
    }
    results.clear_poison(); // Rust >= 1.77: the data is consistent again
    assert_eq!(*results.lock().unwrap(), vec![1.0, 2.0]);
}

#[test]
fn test() {
    main()
}
