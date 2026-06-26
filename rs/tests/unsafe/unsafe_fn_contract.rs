//! `unsafe fn` and the `# Safety` contract.
//!
//! Marking a function `unsafe fn` does **not** make its body magically correct.
//! It does two things:
//! 1. It forces every *caller* to use an `unsafe { ... }` block, acknowledging
//!    that they have read and upheld the function's preconditions.
//! 2. It obliges the *author* to document those preconditions in a `# Safety`
//!    section, so callers know exactly what they must guarantee.
//!
//! The compiler cannot verify a `# Safety` contract — it is a human-to-human
//! agreement. Violating it is Undefined Behavior, even if "it seems to work".

/// Reads a `T` from a raw pointer, bypassing the borrow checker.
///
/// # Safety
/// The caller must guarantee that `ptr`:
/// - is **non-null** and **properly aligned** for `T`,
/// - points to a **single, live allocation** holding an **initialized** `T`,
/// - stays valid for reads for the duration of this call,
/// - is not written through by anyone else during the call (no data race).
///
/// `T: Copy` ensures we do not accidentally duplicate ownership of a non-`Copy`
/// value (which would later double-free).
unsafe fn read_t<T: Copy>(ptr: *const T) -> T {
    // SAFETY: delegated to the caller via the contract above. `read` performs
    // an aligned, typed read; with `Copy` it cannot create an aliasing-ownership
    // hazard.
    *ptr
}

/// Sets `*ptr` to `value`, returning the previous value.
///
/// # Safety
/// `ptr` must be non-null, aligned, point to a live, initialized, **uniquely
/// borrowed** `T`, and valid for both reads and writes for the call.
unsafe fn replace_t<T: Copy>(ptr: *mut T, value: T) -> T {
    let old = *ptr;
    *ptr = value;
    old
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_call_site() {
        let n = 7_u64;
        let p: *const u64 = &n;

        // SAFETY: `p` is derived from `&n`, so it is non-null, aligned, points
        // to a live initialized `u64`, and nothing writes to `n` concurrently.
        // This is exactly what `read_t`'s contract demands.
        let got = unsafe { read_t(p) };
        assert_eq!(got, 7);
    }

    #[test]
    fn replace_through_pointer() {
        let mut slot = 1_i32;
        let p: *mut i32 = &mut slot;

        // SAFETY: `p` is the unique pointer to `slot`, aligned and live; we hold
        // no other reference to `slot` while calling `replace_t`.
        let previous = unsafe { replace_t(p, 99) };

        assert_eq!(previous, 1);
        assert_eq!(slot, 99);
    }
}
