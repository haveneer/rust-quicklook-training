use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    // Example 1: Rc<T> is not Send (not thread safe)
    let rc_counter = Rc::new(RefCell::new(0));
    {
        let rc_counter = rc_counter.clone();
        let f = move || {
            *rc_counter.borrow_mut() += 1;
        };
        // { // Error: `Rc<RefCell<i32>>` cannot be sent between threads safely
        //     let f = thread::spawn(f); f.join().unwrap();
        // } // TODO try to use a thread on it
        for _ in 0..5 {
            f();
        }
    };
    println!("Final rc_counter: {}", *rc_counter.borrow());

    // Example 2: Arc<T> is Send when T: Send + Sync (here thanks to Mutex)
    let arc_counter = Arc::new(Mutex::new(0)); // Arc + Mutex for shared and protected counter
    let mut handles = vec![];
    for _ in 0..5 {
        let counter_cloned = Arc::clone(&arc_counter);
        handles.push(std::thread::spawn(move || {
            // Each thread acquires a mutable lock on the counter before modifying it
            let mut num = counter_cloned.lock().unwrap();
            *num += 1;
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("Final arc_counter: {}", *arc_counter.lock().unwrap());
    // ^ This code compiles and runs correctly because Arc<Mutex<i32>> is Send + Sync.

    // Example 3: RefCell<T> is Send but not Sync
    let cell = RefCell::new(0);
    // std::thread::scope(|s| { // Error: `RefCell<i32>` cannot be shared between threads safely
    //     s.spawn(|| *cell.borrow_mut() += 1);
    // });
    std::thread::spawn(move || *cell.borrow_mut() += 1) // OK: moved, not shared
        .join()
        .unwrap();
}

#[test]
fn test() {
    main()
}
