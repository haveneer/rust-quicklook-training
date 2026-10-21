use std::collections::{BTreeMap, BTreeSet, HashMap};

fn main() {
    let names = ["mesh", "solve", "mesh", "export"];

    // collect: the *target type* decides what is built (FromIterator)
    let v: Vec<String> = names.iter().map(|n| n.to_uppercase()).collect();
    let set: BTreeSet<&str> = names.iter().copied().collect(); // sorted set
    let index: HashMap<&str, usize> = names.iter().enumerate().map(|(i, &n)| (n, i)).collect();
    let initials: String = names.iter().filter_map(|n| n.chars().next()).collect();
    let lengths = names.iter().map(|n| n.len()).collect::<Vec<_>>(); // turbofish
    println!("{v:?}\n{set:?}\n{index:?}\n{initials} {lengths:?}");

    // Results: Ok(Vec) if all succeed, else the first Err (and stop)
    let ok: Result<Vec<i32>, _> = ["1", "2", "3"].iter().map(|s| s.parse::<i32>()).collect();
    let err: Result<Vec<i32>, _> = ["1", "x", "3"].iter().map(|s| s.parse::<i32>()).collect();
    println!("{ok:?} {err:?}"); // same for Option: Some(Vec) or None

    // partition / unzip: two collections in one pass
    let (short, long): (Vec<&str>, Vec<&str>) = names.iter().partition(|n| n.len() <= 4);
    let (ids, lens): (Vec<usize>, Vec<usize>) = names.iter().map(|n| n.len()).enumerate().unzip();
    println!("{short:?} {long:?} {ids:?} {lens:?}");

    // Group by key: entry().or_default() in a BTreeMap (sorted keys) or a HashMap
    let mut by_len: BTreeMap<usize, Vec<&str>> = BTreeMap::new();
    for &n in &names {
        by_len.entry(n.len()).or_default().push(n);
    }

    // In place, no iterator: retain drops, extract_if hands removed items over
    let mut work = names.to_vec();
    let removed: Vec<_> = work.extract_if(.., |n| *n == "mesh").collect();
    assert_eq!((by_len[&4].len(), removed.len(), work), (2, 2, vec!["solve", "export"]));
}

#[test]
fn test() {
    main()
}
