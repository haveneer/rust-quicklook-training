use std::sync::{Arc, Mutex};
// use std::rc::Rc as Arc; // try it: Rc is not Send
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for _ in 0..2 {
        let counter = Arc::clone(&counter); // one owner per thread
        handles.push(thread::spawn(move || {
            for _ in 0..1_000_000 {
                *counter.lock().unwrap() += 1;
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let total = *counter.lock().unwrap();
    println!("counter = {total}");
    assert_eq!(total, 2_000_000);
}

#[test]
fn test() {
    main()
}

#[test]
fn threads_arc_failures() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/failures/stable/threads_rc_not_send.rs");
}
