fn main() {
    // Add
    let mut v = vec![1, 2];
    v.push(3); // at the end: O(1) amortised
    v.insert(0, 0); // anywhere else: shifts the tail, O(n)
    v.extend([4, 5]); // from any iterator, reserves once when the size is known
    let mut tail = vec![6, 7];
    v.append(&mut tail); // moves the elements, `tail` is left empty
    assert_eq!((&v, tail.is_empty()), (&vec![0, 1, 2, 3, 4, 5, 6, 7], true));

    // Remove
    assert_eq!(v.pop(), Some(7)); // last element: O(1), None if empty
    assert_eq!(v.remove(0), 0); // keeps the order, shifts the tail: O(n)
    assert_eq!(v.swap_remove(0), 1); // last element takes its place: O(1), order lost
    assert_eq!(v, [6, 2, 3, 4, 5]);
    v.retain(|&x| x % 2 == 0); // in place, keeps the order, single pass
    let removed: Vec<i32> = v.drain(1..).collect(); // removes a range, yields it
    assert_eq!((&v, removed), (&vec![6], vec![2, 4]));
    let mut w = vec![1, 1, 2, 2, 2, 1];
    w.dedup(); // consecutive duplicates only: sort first for a full dedup
    assert_eq!(w, [1, 2, 1]);
    w.truncate(1); // capacity is kept
    w.clear();
    assert!(w.is_empty() && w.capacity() >= 6);

    // Search
    let v = vec![3, 8, 1, 9, 4];
    assert!(v.contains(&9)); // linear scan: O(n)
    assert_eq!(v.iter().position(|&x| x > 5), Some(1)); // index of the first match
    assert_eq!(v.iter().find(|&&x| x % 2 == 0), Some(&8)); // the element itself
    let mut sorted = v.clone();
    sorted.sort_unstable(); // [1, 3, 4, 8, 9]: binary_search requires sorted data
    assert_eq!(sorted.binary_search(&8), Ok(3)); // found at index 3: O(log n)
    assert_eq!(sorted.binary_search(&5), Err(3)); // absent: where to insert it
    let first_big = sorted.partition_point(|&x| x < 5); // first index with x >= 5
    assert_eq!(&sorted[first_big..], [8, 9]);
}

#[test]
fn test() {
    main()
}
