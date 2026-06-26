//! `static mut` and the modern `&raw` / `addr_of!` access pattern.
//!
//! A `static mut` is a single, global, mutable location. Any access is `unsafe`
//! because nothing stops two threads (or even two code paths) from creating
//! overlapping `&mut` references to it — which is instant UB.
//!
//! The historical footgun was writing `&mut COUNTER` / `&COUNTER`: creating a
//! *reference* to a `static mut` asserts the full borrow rules for that
//! reference's lifetime, which is almost never what you can guarantee. Modern
//! Rust lints this (`static_mut_refs`) and pushes you toward **raw** pointers:
//!
//! - `&raw const PLACE` / `&raw mut PLACE` (stable since 1.82), or the older
//!   macros `std::ptr::addr_of!` / `std::ptr::addr_of_mut!`.
//!
//! These produce a `*const` / `*mut` *without* ever forming an intermediate
//! reference, so you never accidentally claim a borrow you cannot uphold. You
//! still read/write through the raw pointer inside `unsafe`, and you are still
//! responsible for avoiding concurrent access.
//!
//! In real code prefer `AtomicUsize`, `OnceLock`, `Mutex`, etc.; `static mut`
//! is shown here only to explain the hazard. See also `tests/static_and_const.rs`.

static mut COUNTER: u64 = 0;

/// Increments the global counter and returns the new value.
///
/// # Safety
/// Must not be called concurrently from multiple threads: `COUNTER` is a plain
/// `static mut`, so overlapping access is a data race (UB). For real shared
/// state use an atomic or a lock instead.
unsafe fn bump() -> u64 {
    // Form a raw pointer WITHOUT creating a `&mut COUNTER` reference.
    let p: *mut u64 = &raw mut COUNTER;
    // SAFETY: single-threaded use (per the contract); `p` points to the live,
    // aligned, initialized global. No other reference to `COUNTER` exists.
    *p += 1;
    *p
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr::addr_of;

    #[test]
    fn raw_pointer_access_to_static_mut() {
        // SAFETY: this test runs single-threaded and is the only accessor of
        // `COUNTER` here, satisfying `bump`'s contract.
        let a = unsafe { bump() };
        let b = unsafe { bump() };
        assert_eq!(b, a + 1);

        // Reading via `addr_of!` (equivalent to `&raw const`): again, no
        // reference to the `static mut` is ever materialized.
        let p: *const u64 = addr_of!(COUNTER);
        // SAFETY: single-threaded; `p` is valid for reads of the live global.
        let current = unsafe { *p };
        assert_eq!(current, b);
    }
}
