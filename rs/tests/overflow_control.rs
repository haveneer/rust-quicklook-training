#![cfg_attr(feature = "nightly", feature(bigint_helper_methods))]
// Ref: https://huonw.github.io/blog/2016/04/myths-and-legends-about-integer-overflow-in-rust/

use std::ops::Add;

fn main() {
    let x: u8 = 200;
    let y: u8 = 155;

    // x + y; // compile time error: this arithmetic operation will overflow
    // x.add(y); // runtime error: attempt to add with overflow
    assert_eq!(x.checked_add(y), None);
    assert_eq!(x.overflowing_add(y), (99, true));
    assert_eq!(x.saturating_add(y), 255);
    assert_eq!(x.wrapping_add(y), 99);
    #[cfg(feature = "nightly")]
    assert_eq!(x.carrying_add(y, true), (100, true)); // unstable
}

#[test]
fn test() {
    main()
}

#[test]
#[should_panic]
fn test_add_with_overflow() {
    let x: u8 = 200;
    let y: u8 = 155;

    // x + y; // compile time error: this arithmetic operation will overflow
    x.add(y); // runtime error: attempt to add with overflow
}
