use std::collections::VecDeque;

fn main() {
    // Ring buffer: a Vec whose start can move, no shifting at either end
    let mut queue: VecDeque<u32> = VecDeque::with_capacity(4);
    queue.extend([1, 2, 3, 4]);
    assert_eq!(queue.pop_front(), Some(1)); // FIFO: push_back + pop_front
    queue.push_front(0); // O(1), where Vec::insert(0, _) is O(n)
    queue.pop_front();
    queue.pop_front();
    queue.extend([5, 6]); // the start moved: these wrap to the front of the buffer
    assert_eq!(queue[0], 3); // indexed access stays O(1)

    // The content may wrap around the end of the buffer: two slices
    let (head, tail) = queue.as_slices();
    println!("head = {head:?}, tail = {tail:?}");
    let all: &mut [u32] = queue.make_contiguous(); // one slice (may move items)
    all.sort_unstable_by(|a, b| b.cmp(a));
    println!("sorted = {queue:?}");

    // Sliding window over a stream: moving average of the last 3 values
    let mut window = VecDeque::with_capacity(3);
    for x in [1.0, 2.0, 6.0, 4.0, 8.0] {
        if window.len() == 3 {
            window.pop_front();
        }
        window.push_back(x);
        let mean: f64 = window.iter().sum::<f64>() / window.len() as f64;
        print!("{mean:.2} ");
    }
    println!();
}

#[test]
fn test() {
    main()
}
