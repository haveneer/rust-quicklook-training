//! Building a *sound safe abstraction* over `unsafe` internals.
//!
//! This is the whole point of `unsafe`: confine it behind a small, audited
//! boundary so the *public* API can be used freely from safe code without any
//! way to trigger UB. The classic example is `slice::split_at_mut`: returning
//! two `&mut` slices into the same buffer is impossible to express to the borrow
//! checker, yet it is perfectly sound *as long as the two halves do not overlap*.
//!
//! The encapsulation contract here:
//! - The function takes `&mut [T]`, so it has exclusive access to the whole
//!   slice for the duration of the borrow.
//! - It splits at `mid`, producing `[0, mid)` and `[mid, len)` — two regions
//!   that provably do not overlap.
//! - The returned references borrow from the same input lifetime, so the
//!   borrow checker still prevents using the original slice while either half
//!   is alive.
//!
//! Because of those invariants, the `unsafe` block cannot produce aliasing
//! `&mut`, so the API is sound and is exposed *without* the `unsafe` keyword.
//!
//! See also "Learn Rust With Entirely Too Many Linked Lists" for more on
//! wrapping `unsafe` in safe interfaces.

use std::slice;

/// A re-implementation of the standard [`slice::split_at_mut`], by hand.
///
/// Returns two non-overlapping mutable sub-slices: `(&mut s[..mid], &mut s[mid..])`.
///
/// # Panics
/// Panics if `mid > s.len()` (matching the standard library).
///
/// Note: this function is **safe** to call. The `unsafe` is fully contained and
/// justified by the non-overlap invariant.
fn split_at_mut<T>(s: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = s.len();
    let ptr = s.as_mut_ptr();

    // This `assert!` is load-bearing for soundness: it guarantees `mid <= len`,
    // so both ranges below stay in bounds of the original allocation.
    assert!(mid <= len, "mid ({mid}) must be <= len ({len})");

    // SAFETY:
    // - `ptr` is valid for `len` elements (it comes from `s`).
    // - `mid <= len` (just asserted), so `[0, mid)` and `[mid, len)` are both
    //   in-bounds and DO NOT overlap.
    // - We hold `&mut [T]`, i.e. exclusive access, so creating two disjoint
    //   `&mut` views does not alias any other live reference.
    // - The returned slices borrow `s`'s lifetime, so the borrow checker keeps
    //   `s` itself unusable while they are alive.
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_into_two_halves() {
        let mut data = [1, 2, 3, 4, 5, 6];
        let (left, right) = split_at_mut(&mut data, 3);

        assert_eq!(left, &mut [1, 2, 3]);
        assert_eq!(right, &mut [4, 5, 6]);
    }

    #[test]
    fn both_halves_are_independently_mutable() {
        // The key soundness property: we can mutate BOTH halves at the same
        // time. This is impossible to write with two plain `&mut data[..]`
        // borrows, but is sound here because the halves are disjoint.
        let mut data = [10, 20, 30, 40];
        let (left, right) = split_at_mut(&mut data, 2);

        left[0] += 1; // touches index 0
        right[1] += 1; // touches index 3

        assert_eq!(data, [11, 20, 30, 41]);
    }

    #[test]
    fn edge_splits_are_allowed() {
        let mut data = [7, 8, 9];

        let (l, r) = split_at_mut(&mut data, 0);
        assert!(l.is_empty());
        assert_eq!(r, &mut [7, 8, 9]);

        let (l, r) = split_at_mut(&mut data, 3);
        assert_eq!(l, &mut [7, 8, 9]);
        assert!(r.is_empty());
    }

    #[test]
    #[should_panic(expected = "must be <= len")]
    fn out_of_range_mid_panics() {
        let mut data = [1, 2, 3];
        // `mid > len` would make the second range escape the allocation; the
        // safe wrapper rejects it with a panic instead of risking UB.
        let _ = split_at_mut(&mut data, 4);
    }
}
