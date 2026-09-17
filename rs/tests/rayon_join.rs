// What par_iter() does for you: recursive splitting with rayon::join
fn sum(v: &[f64]) -> f64 {
    if v.len() <= 1024 {
        return v.iter().sum(); // small enough: sequential
    }
    let (left, right) = v.split_at(v.len() / 2);
    // `right` is pushed on this worker's deque: an idle worker may steal it
    let (a, b) = rayon::join(|| sum(left), || sum(right));
    a + b
}

fn main() {
    let v: Vec<f64> = (0..1_000_000).map(|i| i as f64).collect();
    let s = sum(&v);
    println!("sum = {s}");
    assert_eq!(s, 499_999_500_000.0);
}

#[test]
fn test() {
    main()
}
