use arrayvec::ArrayVec;
use smallvec::{smallvec, SmallVec};
use std::mem::size_of;

fn main() {
    // Up to 4 elements stored inline (on the stack), beyond that on the heap
    let mut neighbours: SmallVec<[u32; 4]> = smallvec![1, 2, 3];
    neighbours.push(4);
    assert!(!neighbours.spilled()); // still inline: no allocation so far
    neighbours.push(5);
    assert!(neighbours.spilled()); // 5 > 4: moved to the heap, like a Vec

    // Same API as Vec (Deref to a slice)
    neighbours.retain(|n| *n % 2 == 1);
    assert_eq!(&neighbours[..], [1, 3, 5]);

    // The price: the value grows with N, and a branch (inline or heap?) on each access
    assert_eq!(size_of::<Vec<u32>>(), 24); // pointer + capacity + length (64 bits)
    assert_eq!(size_of::<SmallVec<[u32; 4]>>(), 24); // 4 x u32 fit in the same space
    assert_eq!(size_of::<SmallVec<[u32; 16]>>(), 72);

    // ArrayVec: fixed capacity, never allocates; overflowing is an error
    let mut stack: ArrayVec<u32, 4> = ArrayVec::new();
    for i in 0..4 {
        stack.push(i);
    }
    assert!(stack.try_push(4).is_err()); // push() would panic
    assert_eq!(size_of::<ArrayVec<u32, 4>>(), 20); // 4 x u32 + a u32 length
    println!("{neighbours:?} {stack:?}");
}

#[test]
fn test() {
    main()
}
