use std::cell::RefCell;
use std::hint::black_box;

const ITERATIONS: usize = 1_000_000;

// Wrong: black_box on the result only.
// The optimizer sees the whole loop on a local RefCell: the borrow checks
// vanish and the loop folds into the constant 1_000_000.
fn increments_folded() -> usize {
    let cell = RefCell::new(0_usize);
    for _ in 0..ITERATIONS {
        *cell.borrow_mut() += 1;
    }
    black_box(cell.into_inner())
}

// Right: black_box on what each iteration works on.
// The optimizer can no longer track the RefCell: every borrow_mut is checked.
fn increments_measured() -> usize {
    let cell = RefCell::new(0_usize);
    for _ in 0..ITERATIONS {
        *black_box(&cell).borrow_mut() += 1;
    }
    cell.into_inner()
}

#[test]
fn test() {
    assert_eq!(increments_folded(), ITERATIONS);
    assert_eq!(increments_measured(), ITERATIONS);
}
