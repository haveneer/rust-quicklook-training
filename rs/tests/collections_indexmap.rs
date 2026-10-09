use indexmap::IndexMap;
use std::collections::HashMap;

fn main() {
    let stages = [
        ("mesh", 3.2),
        ("assemble", 1.1),
        ("solve", 12.5),
        ("export", 0.4),
    ];

    // HashMap: iteration order depends on the hashes (and on the random seed)
    let hash: HashMap<&str, f64> = stages.into_iter().collect();
    println!("HashMap : {:?}", hash.keys().collect::<Vec<_>>());

    // IndexMap: insertion order, hash lookup *and* access by position
    let mut timings: IndexMap<&str, f64> = stages.into_iter().collect();
    println!("IndexMap: {:?}", timings.keys().collect::<Vec<_>>());
    assert_eq!(timings["solve"], 12.5); // by key, O(1)
    assert_eq!(timings.get_index(0), Some((&"mesh", &3.2))); // by position, O(1)
    assert_eq!(timings.get_index_of("export"), Some(3));

    // Two ways to remove: choose between speed and order
    timings.swap_remove("mesh"); // O(1): the last entry takes its place
    println!("swap_remove : {:?}", timings.keys().collect::<Vec<_>>());
    timings.shift_remove("export"); // O(n): shifts the following entries
    println!("shift_remove: {:?}", timings.keys().collect::<Vec<_>>());

    // Entries are stored in a dense Vec: sortable, and iterated like a Vec
    timings.sort_by(|_, a, _, b| b.total_cmp(a));
    println!("sorted by time: {timings:?}");
}

#[test]
fn test() {
    main()
}
