fn main() {
    let s = [0.1, 0.3, 0.9, 1.4, 0.8, 0.2, 1.1];

    // take / skip, take_while / skip_while: cut by count or by predicate
    let warmup: Vec<f64> = s.iter().copied().take_while(|&x| x < 1.0).collect();
    assert_eq!(warmup, [0.1, 0.3, 0.9]);
    let after: Vec<f64> = s.iter().copied().skip_while(|&x| x < 1.0).skip(1).collect();
    assert_eq!(after, [0.8, 0.2, 1.1]);

    // map_while: map until the first None (stops, unlike filter_map)
    let words = ["1", "2", "x", "4"];
    let ints: Vec<i32> = words.iter().map_while(|w| w.parse().ok()).collect();
    assert_eq!(ints, [1, 2]);

    // step_by: every k-th element; scan: map with a running state (prefix sums)
    assert_eq!((0..10).step_by(3).collect::<Vec<_>>(), [0, 3, 6, 9]);
    let csum: Vec<i32> = (1..=4)
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .collect();
    assert_eq!(csum, [1, 3, 6, 10]);

    // peekable: look at the next item without consuming it (here: a small lexer)
    let mut it = "123abc".chars().peekable();
    let digits: String = std::iter::from_fn(|| it.next_if(char::is_ascii_digit)).collect();
    assert_eq!((digits.as_str(), it.peek()), ("123", Some(&'a')));

    // On slices: windows (overlapping) and chunks (disjoint)
    let deltas: Vec<f64> = s.windows(2).map(|w| w[1] - w[0]).collect();
    let sums: Vec<f64> = s.chunks(3).map(|c| c.iter().sum()).collect();
    println!("{deltas:.1?} {sums:.1?}");
}

#[test]
fn test() {
    main()
}
