use std::{panic, thread};

fn main() {
    let default_hook = panic::take_hook(); // keep the default report to chain to it
    panic::set_hook(Box::new(move |info| {
        // Runs in the panicking thread, *before* unwinding (or abort)
        let payload = info.payload(); // Box<dyn Any>: String or &str for panic!
        let msg = (payload.downcast_ref::<String>().map(String::as_str))
            .or_else(|| payload.downcast_ref::<&str>().copied())
            .unwrap_or("?");
        let line = info.location().map_or(0, |l| l.line());
        let name = thread::current().name().unwrap_or("?").to_owned();
        eprintln!("[solver] {name} panicked line {line}: {msg}");
        default_hook(info); // optional: also print the standard message
    }));

    let worker = thread::Builder::new()
        .name("mesh-reader".into())
        .spawn(|| panic!("cell {} has no node", 42))
        .unwrap();
    assert!(worker.join().is_err()); // the hook did not stop the unwinding

    let _ = panic::take_hook(); // back to the default hook
}

#[test]
fn test() {
    main()
}
