// `Send` is an auto trait the compiler derives conservatively. `Rc<T>` is
// deliberately `!Send` (its reference count is not atomic), so moving it into a
// spawned thread is rejected. This is the *safe* default; the matching positive
// demo `unsafe_send_sync.rs` shows how an author can soundly opt back in with
// `unsafe impl Send` when the invariants actually hold.
use std::rc::Rc;

fn main() {
    let data = Rc::new(5);
    let handle = std::thread::spawn(move || *data); //~ ERROR `Rc<i32>` cannot be sent between threads safely
    println!("{}", handle.join().unwrap());
}
