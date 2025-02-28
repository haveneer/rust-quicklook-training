enum Case {
    A,
    B(i64),
    C,
}

fn without_let_else(s: &str, c: Case) -> i64 {
    let (i, j) = match (s.parse::<i64>(), c) {
        (Ok(i), Case::B(j)) => (i, j),
        _ => panic!("Bad args"),
    };

    i + j
}

fn with_let_else(s: &str, c: Case) -> i64 {
    let (Ok(i), Case::B(j)) = (s.parse::<i64>(), c) else {
        panic!("Bad args")
    };

    i + j
}

fn main() {
    without_let_else("123", Case::B(456));
    with_let_else("123", Case::B(456));
}

#[test]
fn test() {
    main();
}
