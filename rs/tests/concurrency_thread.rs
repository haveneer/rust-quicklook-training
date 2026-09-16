use std::thread;
use std::time::Duration;

fn expensive_job(id: u32) -> u32 {
    for step in 0..3 {
        println!("thread {id} [{:?}]: step {step}", thread::current().id());
        thread::sleep(Duration::from_millis(100));
    }
    id * 10
}

fn main() {
    // Both jobs run at the same time, each on its own OS thread
    let first = thread::spawn(|| expensive_job(1));
    let second = thread::spawn(|| expensive_job(2));

    // join() waits for the thread and gives back its result (Err if it panicked)
    let a = first.join().expect("thread 1 panicked");
    let b = second.join().expect("thread 2 panicked");
    println!("results: {a} and {b}");
}

#[test]
fn test() {
    main()
}

#[test]
fn threads_failures() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/failures/stable/threads_shared_counter.rs");
}
