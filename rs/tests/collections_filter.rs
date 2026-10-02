use std::collections::BTreeMap;

fn main() {
    // (zone, pressure) for 8 cells
    let cells: Vec<(u8, f64)> = (0..8).map(|i| (i % 3, 100.0 + 10.0 * i as f64)).collect();

    // New collection, the source is untouched (borrowed)
    let high: Vec<&(u8, f64)> = cells.iter().filter(|(_, p)| *p > 150.0).collect();
    assert_eq!(high.len(), 2);

    // Two collections in one pass
    let (zone0, others): (Vec<(u8, f64)>, Vec<_>) = cells.iter().partition(|(z, _)| *z == 0);
    assert_eq!((zone0.len(), others.len()), (3, 5));

    // In place: extract_if removes the matches and hands them over (Rust >= 1.87)
    let mut work = cells.clone();
    let removed: Vec<_> = work.extract_if(.., |(_, p)| *p < 120.0).collect();
    assert_eq!((removed.len(), work.len()), (2, 6));

    // Group by key: a BTreeMap of Vec (sorted keys)
    let mut by_zone: BTreeMap<u8, Vec<f64>> = BTreeMap::new();
    for &(zone, p) in &cells {
        by_zone.entry(zone).or_default().push(p);
    }
    println!("{by_zone:?}");
}

#[test]
fn test() {
    main()
}
