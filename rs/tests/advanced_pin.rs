#![allow(dead_code)]
#![allow(clippy::redundant_locals)]
use std::marker::PhantomPinned;
use std::pin::{pin, Pin};

// #[derive(Clone, Copy)] // Be careful: default implementation will break the inner invariant
// Pin doesn't protect your from yourself
struct MyPinnedStruct {
    data: [u8; 4],
    _pin: PhantomPinned, // force to be !Unpin (once pinned, never be able to be unpinned again)
}

// impl Unpin for MyPinnedStruct {} // Allow to Unpin anyway (back to normal)

impl MyPinnedStruct {
    fn new() -> Self {
        Self {
            data: [1, 2, 3, 4],
            _pin: PhantomPinned,
        }
    }
}

mod details {
    use super::*;
    use std::ops::Deref;

    pub fn show<T: Deref<Target = MyPinnedStruct>>(b: &T) {
        println!("{:p}/{:p} : {:?}", b, b.data.as_ptr(), b.data);
    }

    pub fn ref_access(_v: &[u8]) {}
    pub fn mut_access(_v: &mut [u8]) {}

    pub fn move_into_closure<T: Deref<Target = MyPinnedStruct>>(t: T) -> impl FnOnce() {
        move || {
            show(&t); // variable v moved but not content
        }
    }

    // Only to pass &MyPinnedStruct to move_into_closure with too many changes
    // impl Deref for MyPinnedStruct {
    //     type Target = Self;
    //     fn deref(&self) -> &Self::Target {
    //         self
    //     }
    // }
}

fn with_pin_box() -> impl FnOnce() {
    // Box::pin is more or less Pin::new(Box::new()) but able to manage !Unpin value
    // (Pin::new requires Unpin on the value)
    let pin_box: Pin<Box<MyPinnedStruct>> = Box::pin(MyPinnedStruct::new());
    details::show(&pin_box);
    let mut pin_box = pin_box; // move variable not content
    details::show(&pin_box); // variable moved but not the context
    details::ref_access(&pin_box.data); // can be used by simple immutable reference

    // Pin::into_inner(pin_box); // Not allowed on !Unpin value
    details::move_into_closure(pin_box) // move in the closure
}

fn with_pin_value() -> impl FnOnce() {
    // stack data pinned; cannot be move since
    let pin_value: Pin<&mut MyPinnedStruct> = pin!(MyPinnedStruct::new());
    details::show(&pin_value);
    let pin_value = pin_value; // move variable not content
    details::show(&pin_value);

    // let value = *Pin::into_inner(pin_value); // Not allowed on !Unpin value
    // details::move_into_closure(value) // TODO try to return this
    // details::move_into_closure(pin_value) // TODO try to return this
    || {} // to make it functional by default
}

fn main() {
    with_pin_box()(); // sub-functions to force to use other stack frames
    with_pin_value()();
}

#[test]
fn test() {
    main()
}

#[test]
#[rustfmt::skip]
fn pin_failures() {
    let t = trybuild::TestCases::new();

    t.compile_fail(format!("tests/failures/stable/pin_not_mut_self.rs"));
}
