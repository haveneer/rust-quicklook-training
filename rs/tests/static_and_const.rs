use std::sync::atomic::{AtomicUsize, Ordering};

// `const`: a value, copied at each use; no address of its own
const MAX_USERS: usize = 64;
// `static`: one object, fixed address, alive for the whole program
static GREETING: &str = "Hello";

// Interior mutability makes the difference visible
#[allow(clippy::declare_interior_mutable_const)]
const CONST_COUNTER: AtomicUsize = AtomicUsize::new(0);
static STATIC_COUNTER: AtomicUsize = AtomicUsize::new(0);

struct Buffer {
    data: [u8; Self::SIZE],
}

impl Buffer {
    const SIZE: usize = 64; // associated const
}

#[allow(clippy::borrow_interior_mutable_const)]
fn main() {
    for _ in 0..3 {
        CONST_COUNTER.fetch_add(1, Ordering::Relaxed); // a fresh copy
        STATIC_COUNTER.fetch_add(1, Ordering::Relaxed); // the only one
    }
    println!("const : {}", CONST_COUNTER.load(Ordering::Relaxed));
    println!("static: {}", STATIC_COUNTER.load(Ordering::Relaxed));

    println!("static: {:p} {:p}", &STATIC_COUNTER, &STATIC_COUNTER);
    println!("const : {:p} {:p}", &CONST_COUNTER, &CONST_COUNTER);

    let buffer = Buffer {
        data: [0; Buffer::SIZE],
    };
    assert_eq!(buffer.data.len(), MAX_USERS);
    println!("{GREETING}, {} users max", MAX_USERS);
}

#[test]
fn test() {
    main()
}
