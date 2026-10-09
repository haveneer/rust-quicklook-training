use hashbrown::{HashMap, HashTable};
use std::hash::{BuildHasher, RandomState};

fn main() {
    let text = "to be or not to be";

    // Same SwissTable as std::collections::HashMap, but foldhash by default
    let mut count: HashMap<String, usize> = HashMap::new();
    for word in text.split(' ') {
        // entry_ref borrows the key: a String is allocated only if it is missing
        // (std's entry(word.to_owned()) allocates on every call)
        *count.entry_ref(word).or_insert(0) += 1;
    }
    println!("{count:?}");

    // Opt back into std's SipHash (DoS resistant) when keys come from outside
    let safe: HashMap<&str, u32, RandomState> = HashMap::from_iter([("mesh", 1)]);
    assert_eq!(safe["mesh"], 1);

    // HashTable: the raw table, you provide hash and equality (no K/V split)
    let state = RandomState::new();
    let mut names: HashTable<(u32, &str)> = HashTable::new();
    for (id, name) in [(7, "wing"), (3, "tail")] {
        let hash = state.hash_one(name);
        names.insert_unique(hash, (id, name), |&(_, n)| state.hash_one(n));
    }
    // lookup by the name stored inside the entry, no separate key copy
    let hit = names.find(state.hash_one("tail"), |&(_, n)| n == "tail");
    assert_eq!(hit, Some(&(3, "tail")));
}

#[test]
fn test() {
    main()
}
