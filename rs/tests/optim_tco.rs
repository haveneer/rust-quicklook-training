/// Tail-recursive: the recursive call is the last operation...
fn sum_rec(n: u64, acc: u64) -> u64 {
    if n == 0 {
        acc
    } else {
        sum_rec(n - 1, acc + n)
    }
}

/// ...but Rust does not guarantee that it becomes a loop: write the loop
fn sum_loop(mut n: u64) -> u64 {
    let mut acc = 0;
    while n > 0 {
        acc += n;
        n -= 1;
    }
    acc
}

/// Or let an iterator express it
fn sum_iter(n: u64) -> u64 {
    (1..=n).sum()
}

fn main() {
    let n = 1_000; // small: each recursive call may use a stack frame
    assert_eq!(sum_rec(n, 0), sum_loop(n));
    assert_eq!(sum_loop(n), sum_iter(n));
    println!("sum(1..={n}) = {}", sum_iter(n));
}

#[test]
fn test() {
    main()
}
