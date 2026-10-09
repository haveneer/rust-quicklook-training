use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    // Max-heap: the greatest element is always on top
    let mut heap = BinaryHeap::from([3, 1, 4, 1, 5]);
    heap.push(9); // O(1) on average, O(log n) worst case
    assert_eq!(heap.peek(), Some(&9)); // O(1)
    assert_eq!(heap.pop(), Some(9)); // O(log n)
    println!("internal order: {heap:?}"); // a heap, not a sorted array

    // Min-heap: wrap the elements in Reverse
    let mut min_heap: BinaryHeap<Reverse<u32>> = [3, 1, 4].into_iter().map(Reverse).collect();
    assert_eq!(min_heap.pop(), Some(Reverse(1)));

    // Priority queue: tuples compare field by field, priority first
    let mut tasks = BinaryHeap::new();
    tasks.push((2, "mesh"));
    tasks.push((5, "solve"));
    tasks.push((1, "export"));
    while let Some((priority, task)) = tasks.pop() {
        print!("{task}({priority}) ");
    }
    println!();

    // Heap sort: drain in sorted order
    let sorted = BinaryHeap::from([3, 1, 4, 1, 5]).into_sorted_vec();
    assert_eq!(sorted, [1, 1, 3, 4, 5]);
}

#[test]
fn test() {
    main()
}
