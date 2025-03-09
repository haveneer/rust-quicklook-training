use std::num::{Saturating, Wrapping};

fn main() {
    let x = Wrapping(100_i8);
    let y = Wrapping(100_i8);

    assert_eq!(x + y, Wrapping(-56));

    let max = Saturating(u32::MAX);
    let one = Saturating(1u32);
    assert_eq!(u32::MAX, (max + one).0);
}

#[test]
fn test() {
    main()
}
