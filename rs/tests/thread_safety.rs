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
        // { // Error: Rc<RefCell<i32>>` cannot be sent between threads safely
        //     let f = thread::spawn(f); f.join().unwrap();
        // } // TODO try to use a thread on it
        for _ in 0..5 {
            f();
        }
    };
    println!("Final rc_counter: {}", *rc_counter.borrow());

    // Example 2: Arc is Send (thread-safe)
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
}

#[test]
fn test() {
    main()
}
