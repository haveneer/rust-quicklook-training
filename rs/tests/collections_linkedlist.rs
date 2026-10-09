use std::collections::LinkedList;

fn main() {
    // Doubly linked list: one heap allocation per node
    let mut list: LinkedList<u32> = (1..=3).collect();
    list.push_front(0);
    list.push_back(4);
    assert_eq!(list.front(), Some(&0));
    // no list[2]: reaching an element walks the nodes, O(n)
    assert_eq!(list.iter().nth(2), Some(&2));

    // What it does in O(1): split and splice, without moving any element
    let mut tail = list.split_off(3); // O(min(i, n - i)) to reach the node, then O(1)
    println!("list = {list:?}, tail = {tail:?}");
    tail.append(&mut list); // O(1): relinks the nodes
    println!("tail = {tail:?}, list = {list:?}");

    // Same operations on a Vec: elements are moved, O(n)...
    let mut v: Vec<u32> = (0..=4).collect();
    let mut v_tail = v.split_off(3);
    v_tail.append(&mut v);
    // ...but a memcpy of contiguous data usually beats chasing pointers
    assert_eq!(v_tail, [3, 4, 0, 1, 2]);
}

#[test]
fn test() {
    main()
}
