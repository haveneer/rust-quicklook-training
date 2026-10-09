fn main() {
    let speeds = [12.0, 48.5, 33.0, 7.5, 48.5];

    // Reductions: sum / product, fold (initial value + step), reduce (no initial value)
    let total: f64 = speeds.iter().sum();
    assert_eq!(total, 149.5);
    assert_eq!((1..=5).product::<u64>(), 120);
    let energy = speeds.iter().fold(0.0, |acc, v| acc + v * v);
    assert_eq!(energy, 5993.75);
    let fastest = speeds.iter().copied().reduce(f64::max);
    assert_eq!(fastest, Some(48.5)); // None for an empty iterator

    // min / max need Ord (f64 is only PartialOrd): use the *_by / *_by_key variants
    let longest = ["wing", "fuselage", "tail"]
        .into_iter()
        .max_by_key(|s| s.len());
    assert_eq!(longest, Some("fuselage"));
    let slowest = speeds.iter().min_by(|a, b| a.total_cmp(b));
    assert_eq!(slowest, Some(&7.5));

    // Short-circuiting searches: stop at the first answer
    assert!(speeds.iter().any(|&v| v > 40.0));
    assert!(speeds.iter().all(|&v| v > 0.0));
    assert_eq!(speeds.iter().position(|&v| v > 40.0), Some(1));
    assert_eq!(speeds.iter().rposition(|&v| v > 40.0), Some(4));
    assert_eq!(speeds.iter().find(|&&v| v < 10.0), Some(&7.5));

    // Counting and picking
    assert_eq!(speeds.iter().filter(|&&v| v > 30.0).count(), 3);
    let mut it = speeds.iter();
    assert_eq!(it.nth(1), Some(&48.5)); // consumes the first 2 items...
    assert_eq!(it.next(), Some(&33.0)); // ...the iterator goes on from there
    assert_eq!(it.last(), Some(&48.5));
}

#[test]
fn test() {
    main()
}
