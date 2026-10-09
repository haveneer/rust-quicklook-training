use hashbrown::{hash_map::Entry, HashMap};
use std::hash::Hash;

// Natural version, rejected by the current borrow checker (NLL "problem case #3"):
// the `&mut *map` borrowed by the first `get_mut` is returned on one branch,
// so it is considered alive until the end of the function, even on the other branch.
//
// fn get_priority_mut<K: Eq + Hash, V>(map: &mut HashMap<K, V>, key1: K, key2: K) -> Option<&mut V> {
//     if let Some(v) = map.get_mut(&key1) {
//         return Some(v);
//     }
//     map.get_mut(&key2) // error[E0499]: cannot borrow `*map` as mutable more than once
// }
// (accepted by Polonius, the next-generation borrow checker)

fn get_priority_mut<K: Eq + Hash, V>(map: &mut HashMap<K, V>, key1: K, key2: K) -> Option<&mut V> {
    match map.entry(key1) {
        // into_mut: the entry is consumed, the &mut V lives as long as the map borrow
        Entry::Occupied(e) => Some(e.into_mut()),
        Entry::Vacant(e) => {
            // into_map: the vacant entry is consumed and gives back the &mut HashMap
            // it was holding, so the same exclusive borrow continues: no second borrow
            match e.into_map().entry(key2) {
                Entry::Occupied(e) => Some(e.into_mut()),
                Entry::Vacant(_) => None,
            }
        }
    }
}

#[test]
fn test() {
    let mut map = HashMap::from([("fallback", 1)]);
    *get_priority_mut(&mut map, "custom", "fallback").unwrap() += 10;
    map.insert("custom", 5);
    *get_priority_mut(&mut map, "custom", "fallback").unwrap() += 10;
    assert_eq!((map["custom"], map["fallback"]), (15, 11));
    assert!(get_priority_mut(&mut map, "a", "b").is_none());
}
