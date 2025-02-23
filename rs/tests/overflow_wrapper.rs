#![cfg_attr(feature = "nightly", feature(saturating_int_impl))]

use std::num::Wrapping;

fn main() {
    let x = Wrapping(100_i8);
    let y = Wrapping(100_i8);

    assert_eq!(x + y, Wrapping(-56));

    #[cfg(feature = "nightly")]
    let max = Saturating(u32::MAX);
    #[cfg(feature = "nightly")]
    let one = Saturating(1u32);
    #[cfg(feature = "nightly")]
    assert_eq!(u32::MAX, (max + one).0);
}

#[test]
fn test() {
    main()
}
