// cargo run --example debug_crash -- overflow   (stack overflow: SIGABRT, no panic)
// cargo run --example debug_crash -- segfault   (invalid access in unsafe: SIGSEGV)
// cargo run --example debug_crash -- abort      (explicit abort: SIGABRT)
fn depth(n: u64) -> u64 {
    let frame = std::hint::black_box([n; 64]); // 512 bytes per call
    if n == 0 {
        0
    } else {
        1 + depth(frame[0] - 1)
    }
}

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("overflow") => println!("{}", depth(10_000_000)),
        Some("segfault") => unsafe {
            let p = std::ptr::without_provenance_mut::<u64>(0x10); // invalid
            *p = 42;
        },
        Some("abort") => std::process::abort(),
        _ => eprintln!("usage: debug_crash overflow|segfault|abort"),
    }
}
