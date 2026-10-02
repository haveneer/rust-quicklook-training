use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};
use std::thread;

fn main() {
    // The state (task queue, finished flag) lives in the Mutex; the Condvar only signals
    let state = Mutex::new((VecDeque::new(), false));
    let changed = Condvar::new();

    thread::scope(|s| {
        for id in 0..2 {
            let (state, changed) = (&state, &changed);
            s.spawn(move || {
                loop {
                    // Releases the lock while asleep; re-checks the predicate on every wake-up
                    let mut guard = changed
                        .wait_while(state.lock().unwrap(), |(q, done)| q.is_empty() && !*done)
                        .unwrap();
                    match guard.0.pop_front() {
                        Some(task) => println!("worker {id} runs task {task}"),
                        None => break, // empty and finished
                    }
                }
            });
        }
        for task in 1..=4 {
            state.lock().unwrap().0.push_back(task); // modify under the lock...
            changed.notify_one(); // ...then wake one waiting worker
        }
        state.lock().unwrap().1 = true;
        changed.notify_all(); // concerns everybody: let them all see "finished"
    });
}

#[test]
fn test() {
    main()
}
