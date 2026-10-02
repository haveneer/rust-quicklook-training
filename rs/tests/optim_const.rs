/// Evaluated by the compiler when used in a const context
const fn factorials<const N: usize>() -> [u64; N] {
    let mut table = [1; N];
    let mut i = 1;
    while i < N {
        // no `for` (iterators) in const fn yet: plain loops only
        table[i] = table[i - 1] * i as u64;
        i += 1;
    }
    table
}

/// Computed at compile time, stored in the binary: no runtime cost
const FACTORIALS: [u64; 21] = factorials();

/// Const generic: the dimension is part of the type, loops have a known length
fn dot<const D: usize>(a: &[f64; D], b: &[f64; D]) -> f64 {
    const { assert!(D > 0, "empty vectors") }; // checked at compile time (Rust >= 1.79)
    (0..D).map(|i| a[i] * b[i]).sum()
}

fn main() {
    println!("20! = {}", FACTORIALS[20]);
    let (u, v) = ([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]);
    // D = 3, deduced from the arrays
    println!("u.v = {}", dot(&u, &v));
    // dot::<0>(&[], &[]); // error[E0080]: evaluation panicked: empty vectors
}

#[test]
fn test() {
    main()
}
