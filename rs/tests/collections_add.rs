use std::collections::{BTreeSet, HashMap, VecDeque};

fn main() {
    let mut v = vec![1.0, 2.0];
    v.push(3.0); // at the end: O(1) amortised
    v.insert(0, 0.5); // anywhere else: shifts the tail, O(n)
    v.extend([4.0, 5.0]); // from any iterator, reserves once when the size is known
    let mut tail = vec![6.0, 7.0];
    v.append(&mut tail); // moves the elements, `tail` is left empty
    assert_eq!(v, [0.5, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
    assert!(tail.is_empty());

    let mut queue = VecDeque::from([2, 3]);
    queue.push_front(1); // O(1) at both ends, unlike Vec::insert(0, _)
    queue.push_back(4);
    assert_eq!(queue, [1, 2, 3, 4]);

    let mut set = BTreeSet::new();
    assert!(set.insert("mesh")); // true: newly added
    assert!(!set.insert("mesh")); // false: already there, nothing changes

    let mut counts: HashMap<&str, usize> = HashMap::new();
    for word in ["wing", "tail", "wing"] {
        *counts.entry(word).or_insert(0) += 1; // one lookup, insert if missing
    }
    let old = counts.insert("tail", 10); // insert replaces and returns the old value
    assert_eq!((counts["wing"], old), (2, Some(1)));
}

#[test]
fn test() {
    main()
}
