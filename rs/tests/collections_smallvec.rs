use arrayvec::ArrayVec;
use smallvec::{SmallVec, smallvec};

fn main() {
    // Up to 4 elements stored inline (in the value itself), beyond that on the heap
    let mut neighbours: SmallVec<[u32; 4]> = smallvec![1, 2, 3];
    neighbours.push(4);
    assert!(!neighbours.spilled()); // still inline: no allocation so far
    neighbours.push(5);
    assert!(neighbours.spilled()); // 5 > 4: moved to the heap, like a Vec

    // Same API as Vec (Deref to a slice); once spilled, it stays on the heap...
    neighbours.retain(|n| *n % 2 == 1);
    assert_eq!(&neighbours[..], [1, 3, 5]);
    assert!(neighbours.spilled());
    neighbours.shrink_to_fit(); // ...unless shrunk: back inline
    assert!(!neighbours.spilled());

    // The price: the value grows with N, and a branch (inline or heap?) on each access
    assert_eq!(std::mem::size_of::<Vec<u32>>(), 24); // pointer + capacity + length (64 bits)
    assert_eq!(std::mem::size_of::<SmallVec<[u32; 4]>>(), 24); // with the `union` feature (else 32)
    assert_eq!(std::mem::size_of::<SmallVec<[u32; 16]>>(), 72);

    // ArrayVec: fixed capacity, never allocates; overflowing is an error
    let mut stack: ArrayVec<u32, 4> = ArrayVec::new();
    stack.extend(0..4);
    assert!(stack.try_push(4).is_err()); // push() would panic
    assert_eq!(std::mem::size_of::<ArrayVec<u32, 4>>(), 20); // 4 x u32 + a u32 length
    println!("{neighbours:?} {stack:?}");
}

#[test]
fn test() {
    main()
}
