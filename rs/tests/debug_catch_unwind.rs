use std::panic;
use std::thread;

fn fragile(x: i32) -> i32 {
    if x < 0 {
        panic!("negative input: {x}");
    }
    x * 2
}

fn main() {
    // A panic in a thread only kills that thread: join() reports it as an Err
    let handle = thread::spawn(|| fragile(-1));
    match handle.join() {
        Ok(v) => println!("thread returned {v}"),
        Err(payload) => println!("thread panicked: {:?}", payload.downcast_ref::<String>()),
    }

    // catch_unwind stops the unwinding at a boundary (e.g. FFI, plugin, task runner)
    let result = panic::catch_unwind(|| fragile(-2));
    assert!(result.is_err());
    let result = panic::catch_unwind(|| fragile(21));
    assert_eq!(result.ok(), Some(42));
    // Not a try/catch: with panic = "abort", nothing is caught
}

#[test]
fn test() {
    main()
}
