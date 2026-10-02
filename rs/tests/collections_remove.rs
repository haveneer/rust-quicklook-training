use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = (1..=10).collect();
    assert_eq!(v.pop(), Some(10)); // last element: O(1), None if empty
    assert_eq!(v.remove(0), 1); // keeps the order, shifts the tail: O(n)
    assert_eq!(v.swap_remove(0), 2); // last element takes its place: O(1), order lost
    assert_eq!(v, [9, 3, 4, 5, 6, 7, 8]);

    v.retain(|&x| x % 2 == 1); // in place, keeps the order, single pass
    assert_eq!(v, [9, 3, 5, 7]);

    let removed: Vec<i32> = v.drain(1..3).collect(); // removes a range, yields it
    assert_eq!((removed, &v), (vec![3, 5], &vec![9, 7]));

    let mut w = vec![1, 1, 2, 2, 2, 1];
    w.dedup(); // consecutive duplicates only: sort first for a full dedup
    assert_eq!(w, [1, 2, 1]);
    w.truncate(1); // capacity is kept
    w.clear();
    assert!(w.is_empty() && w.capacity() >= 6);

    let mut ids = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    assert_eq!(ids.remove("a"), Some(1));
    ids.retain(|_, v| *v > 2);
    assert_eq!(ids.len(), 1);
}

#[test]
fn test() {
    main()
}
