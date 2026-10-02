use std::collections::{BTreeMap, HashMap};

fn main() {
    let v = vec![3, 8, 1, 9, 4];
    assert!(v.contains(&9)); // linear scan: O(n)
    assert_eq!(v.iter().position(|&x| x > 5), Some(1)); // index of the first match
    assert_eq!(v.iter().find(|&&x| x % 2 == 0), Some(&8)); // the element itself
    assert_eq!(v.iter().max(), Some(&9));

    let mut sorted = v.clone();
    sorted.sort_unstable(); // [1, 3, 4, 8, 9]: binary_search requires sorted data
    assert_eq!(sorted.binary_search(&8), Ok(3)); // found at index 3: O(log n)
    assert_eq!(sorted.binary_search(&5), Err(3)); // absent: where to insert it
    let first_big = sorted.partition_point(|&x| x < 5); // first index with x >= 5
    assert_eq!(&sorted[first_big..], [8, 9]);

    let mut by_name = HashMap::from([("naca0012", 1200), ("rae2822", 800)]);
    assert_eq!(by_name.get("rae2822"), Some(&800)); // O(1) average, Option
    if let Some(n) = by_name.get_mut("naca0012") {
        *n += 1;
    }

    // BTreeMap keeps the keys sorted: range queries in O(log n + k)
    let snapshots = BTreeMap::from([(0, "init"), (100, "s1"), (200, "s2"), (300, "s3")]);
    let window: Vec<_> = snapshots.range(150..=300).map(|(_, v)| *v).collect();
    assert_eq!(window, ["s2", "s3"]);
}

#[test]
fn test() {
    main()
}
