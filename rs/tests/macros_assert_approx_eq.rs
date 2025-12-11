#![allow(clippy::approx_constant)]

macro_rules! assert_approx_eq {
    ($left:expr, $right:expr, $epsilon:expr) => {{
        let diff = f64::abs($left - $right);
        if diff > $epsilon {
            panic!(
                "assertion failed: \
                     `(left ≈ right)` \
                     left: `{:?}`, right: `{:?}`, \
                     diff: `{:?}`, epsilon: `{:?}`",
                $left, $right, diff, $epsilon
            );
        }
    }};
}

fn main() {
    assert_approx_eq!(3.14, 3.14159, 0.01);
    println!("Test passed!");

    // assert_approx_eq!(3.14, 3.2, 0.01); // Panic!
}

#[test]
fn test() {
    main()
}
