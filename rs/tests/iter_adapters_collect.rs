use std::collections::{BTreeSet, HashMap};

fn main() {
    let names = ["mesh", "solve", "mesh", "export"];

    // collect: the *target type* decides what is built (FromIterator)
    let v: Vec<String> = names.iter().map(|n| n.to_uppercase()).collect();
    let set: BTreeSet<&str> = names.iter().copied().collect(); // sorted, deduplicated
    let index: HashMap<&str, usize> = names.iter().enumerate().map(|(i, &n)| (n, i)).collect();
    let initials: String = names.iter().filter_map(|n| n.chars().next()).collect();
    println!("{v:?}\n{set:?}\n{index:?}\n{initials}");

    // The type can also be given with the turbofish
    let lengths = names.iter().map(|n| n.len()).collect::<Vec<_>>();
    assert_eq!(lengths, [4, 5, 4, 6]);

    // Collecting Results: Ok(Vec) if all succeed, otherwise the first Err (and stop)
    let ok: Result<Vec<i32>, _> = ["1", "2", "3"].iter().map(|s| s.parse::<i32>()).collect();
    let err: Result<Vec<i32>, _> = ["1", "x", "3"].iter().map(|s| s.parse::<i32>()).collect();
    println!("{ok:?} {err:?}");
    // Same for Option: Some(Vec) or None
    let all: Option<Vec<char>> = names.iter().map(|n| n.chars().nth(4)).collect();
    assert_eq!(all, None); // "mesh" has no 5th char

    // unzip: one iterator of pairs, two collections
    let (short, long): (Vec<&str>, Vec<&str>) = names.iter().partition(|n| n.len() <= 4);
    let (ids, lens): (Vec<usize>, Vec<usize>) = names.iter().map(|n| n.len()).enumerate().unzip();
    println!("{short:?} {long:?} {ids:?} {lens:?}");
}

#[test]
fn test() {
    main()
}
