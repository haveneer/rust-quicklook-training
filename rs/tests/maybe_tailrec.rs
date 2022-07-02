// neither tail recursion nor tail call optimization are ever guaranteed by Rust,
// although the optimizer may choose to perform them.
// https://stackoverflow.com/a/59258170/12430075

// This example is explicitly tailrec and it is optimized by Rust
fn maybe_tailrec1(n: u64, acc: u64) -> u64 {
    if n == 0 {
        acc
    } else {
        maybe_tailrec1(n - 1, acc + 1)
    }
}

// This example is not explicitly tailrec and nevertheless it is optimized by Rust
fn maybe_tailrec2(n: u64) -> u64 {
    if n == 0 {
        0
    } else {
        1 + maybe_tailrec2(n - 1)
    }
}

fn main() {
    // If they were not optimized, these tests should blow up stack
    println!("{}", maybe_tailrec1(10000, 0));
    println!("{}", maybe_tailrec2(10000));
}

#[test]
fn test() {
    main();
}
