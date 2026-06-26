//! Raw pointers: creating them is safe, *dereferencing* them is not.
//!
//! Creating a `*const T` / `*mut T` is a perfectly safe operation — a raw
//! pointer is just an address (plus *provenance*, see below). The compiler only
//! requires an `unsafe` block when you actually **dereference** it, because at
//! that point *you* are promising the pointer satisfies every invariant the
//! compiler can no longer check.
//!
//! Author's obligations before dereferencing a raw pointer (see the Reference,
//! "Behavior considered undefined"):
//! - **non-null**,
//! - **aligned** for `T`,
//! - **dereferenceable**: points inside a single live allocation for `size_of::<T>()`,
//! - **valid provenance**: derived from a pointer that is allowed to access that
//!   allocation (a raw integer cast back to a pointer does *not* invent access
//!   rights — see `std::ptr` "Provenance"),
//! - the pointee is a **valid `T`** when read.

#[cfg(test)]
mod tests {
    #[test]
    fn create_is_safe_deref_is_unsafe() {
        let x = 42_i32;

        // Creating raw pointers is safe: no `unsafe` needed here.
        let p: *const i32 = &x;
        let p_mut: *mut i32 = &x as *const i32 as *mut i32; // (we won't write through this one)

        // Dereferencing requires `unsafe`: we vouch that `p` is non-null,
        // aligned, points to a live, initialized `i32`, and has provenance for
        // `x`. All true here because `p` was just derived from `&x`.
        let read_back = unsafe { *p };
        assert_eq!(read_back, 42);

        // The address is the same; only access rights (provenance) matter.
        assert_eq!(p, p_mut.cast_const());
    }

    #[test]
    fn write_through_mut_pointer() {
        let mut value = 10_u8;

        let p: *mut u8 = &mut value;

        // SAFETY: `p` came straight from `&mut value`, so it is the unique,
        // aligned, in-bounds pointer to a live `u8`. No other reference is
        // alive while we write through it.
        unsafe {
            *p = 200;
        }

        assert_eq!(value, 200);
    }

    #[test]
    fn offset_within_an_array() {
        let arr = [1_u32, 2, 3, 4];
        let base: *const u32 = arr.as_ptr();

        // SAFETY: indices 0..len stay inside the same allocation, so each
        // `add` produces an in-bounds, aligned pointer that we then read.
        let third = unsafe {
            let p = base.add(2); // pointer arithmetic stays *within* `arr`
            *p
        };
        assert_eq!(third, 3);

        // NOTE: `base.add(arr.len())` (one-past-the-end) is a *valid pointer*
        // to form, but reading it would be out-of-bounds UB. Pointer
        // provenance forbids "escaping" the original allocation.
    }
}
