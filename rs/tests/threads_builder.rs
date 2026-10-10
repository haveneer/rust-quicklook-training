use std::thread;
use std::time::Duration;

fn main() {
    // How many threads can really run at the same time here?
    let n = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    println!("available parallelism: {n}");

    let handles: Vec<_> = (0..n)
        .map(|i| {
            thread::Builder::new()
                .name(format!("worker-{i}")) // shown in panic messages and debuggers
                .stack_size(8 * 1024 * 1024) // default: 2 MiB
                .spawn(move || {
                    let me = thread::current();
                    println!("{} {:?}", me.name().unwrap(), me.id());
                    thread::sleep(Duration::from_millis(10));
                    i * i
                })
                .expect("the OS refused to create a thread") // io::Result
        })
        .collect();

    let total: usize = handles.into_iter().map(|h| h.join().unwrap()).sum();
    println!("total = {total}");
}

#[test]
fn test() {
    main()
}
