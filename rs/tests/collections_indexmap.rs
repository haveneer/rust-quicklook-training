use indexmap::IndexMap;
use std::collections::HashMap;

fn main() {
    #[rustfmt::skip]
    let steps = [("fetch", 1.1), ("compile", 12.5), ("link", 3.2), ("test", 0.4)];

    // HashMap: iteration order depends on the hashes (and on the random seed)
    let hash: HashMap<&str, f64> = steps.into_iter().collect();
    println!("HashMap     : {:?}", hash.keys().collect::<Vec<_>>());

    // IndexMap: insertion order, hash lookup *and* access by position
    let mut timings: IndexMap<&str, f64> = steps.into_iter().collect();
    assert_eq!(timings["compile"], 12.5); // by key, O(1)
    assert_eq!(timings.get_index(0), Some((&"fetch", &1.1))); // by position, O(1)
    assert_eq!(timings.get_index_of("test"), Some(3));

    // Two ways to remove (plain `remove` is deprecated): speed or order
    timings.shift_remove("compile"); // O(n): the following entries shift
    println!("shift_remove: {:?}", timings.keys().collect::<Vec<_>>());
    timings.swap_remove("fetch"); // O(1): the last entry takes its place
    println!("swap_remove : {:?}", timings.keys().collect::<Vec<_>>());

    // Entries are stored in a dense Vec: sortable, and iterated like a Vec
    timings.sort_by(|_, a, _, b| b.total_cmp(a)); // descending time
    println!("sorted      : {timings:?}");
}

#[test]
fn test() {
    main()
}
