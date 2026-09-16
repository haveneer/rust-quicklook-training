use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let cloned = Rc::clone(&counter);
    let handle = thread::spawn(move || {
        *cloned.lock().unwrap() += 1;
    });
    handle.join().unwrap();
    println!("counter = {}", counter.lock().unwrap());
}
