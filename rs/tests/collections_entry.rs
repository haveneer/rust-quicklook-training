use std::collections::btree_map::{BTreeMap, Entry};

fn main() {
    let message = "she sells sea shells by the sea shore";
    // Naive: up to 3 lookups for the same key (contains_key, then get_mut or insert)
    let mut naive: BTreeMap<char, usize> = BTreeMap::new();
    for c in message.chars() {
        if naive.contains_key(&c) {
            *naive.get_mut(&c).unwrap() += 1;
        } else {
            naive.insert(c, 1);
        }
    }

    // Entry: a single lookup, then act on the slot found
    let mut count: BTreeMap<char, usize> = BTreeMap::new();
    for c in message.chars() {
        *count.entry(c).or_insert(0) += 1; // or: .or_default()
    }
    assert_eq!(count, naive);

    // and_modify + or_insert: update if present, insert otherwise (or_insert_with: lazily)
    let mut seen: BTreeMap<char, (usize, usize)> = BTreeMap::new();
    for (i, c) in message.chars().enumerate() {
        seen.entry(c).and_modify(|p| p.1 += 1).or_insert((i, 1));
    }
    println!("{count:?}\n's' (first index, count): {:?}", seen[&'s']);

    // Full control: match on the slot
    match count.entry('z') {
        Entry::Occupied(slot) => println!("'z' seen {} times", slot.get()),
        Entry::Vacant(slot) => println!("no 'z', set to {}", slot.insert(0)),
    }

    // insert_entry: insert or overwrite, and keep the occupied slot (no new lookup)
    // (stable for HashMap; for BTreeMap since Rust 1.92)
    let mut ids: std::collections::HashMap<char, usize> = count.into_iter().collect();
    let slot = ids.entry('z').insert_entry(1);
    println!("{:?} -> {}", slot.key(), slot.get());
}

#[test]
fn test() {
    main()
}
