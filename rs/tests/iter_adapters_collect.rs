use std::collections::{BTreeMap, BTreeSet, HashMap};

fn main() {
    let names = ["mesh", "solve", "mesh", "export"];

    // collect: the *target type* decides what is built (FromIterator)
    let v: Vec<String> = names.iter().map(|n| n.to_uppercase()).collect();
    let set: BTreeSet<&str> = names.iter().copied().collect(); // sorted set
    let index: HashMap<&str, usize> = names.into_iter().zip(0..).collect();
    let first: String = names.iter().filter_map(|n| n.chars().next()).collect();
    let lengths = names.iter().map(|n| n.len()).collect::<Vec<_>>(); // turbofish
    println!("{v:?}\n{set:?}\n{index:?}\n{first} {lengths:?}");

    // Results: Ok(Vec) if all succeed, else the first Err (and stop)
    let ok: Result<Vec<i32>, _> = ["1", "2"].iter().map(|s| s.parse()).collect();
    let err: Result<Vec<i32>, _> = ["1", "x"].iter().map(|s| s.parse()).collect();
    println!("{ok:?} {err:?}"); // same for Option: Some(Vec) or None

    // partition / unzip: two collections in one pass
    let (short, big): (Vec<&str>, _) = names.iter().partition(|n| n.len() < 5);
    let pairs = names.map(|n| (n.len(), n));
    let (lens, ws): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
    println!("{short:?} {big:?} {lens:?} {ws:?}");

    // Group by key: entry().or_default() in a BTreeMap (sorted keys) or a HashMap
    let mut by_len: BTreeMap<usize, Vec<&str>> = BTreeMap::new();
    for &n in &names {
        by_len.entry(n.len()).or_default().push(n);
    }

    // In place, no iterator: retain drops, extract_if hands removed items over
    let mut work = names.to_vec();
    let removed: Vec<_> = work.extract_if(.., |n| *n == "mesh").collect();
    assert_eq!(by_len[&4].len(), 2);
    assert_eq!((removed.len(), work), (2, vec!["solve", "export"]));
}

#[test]
fn test() {
    main()
}
