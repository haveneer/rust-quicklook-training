//! `unsafe trait` / `unsafe impl Send` & `Sync`: promising thread-safety by hand.
//!
//! `Send` and `Sync` are **auto traits**: the compiler derives them structurally
//! and *conservatively*. A type containing a raw pointer (`*const T` / `*mut T`)
//! is automatically `!Send + !Sync`, because a raw pointer carries no aliasing or
//! thread-safety guarantees.
//!
//! When you *know* a type is actually thread-safe, you can override that decision
//! with `unsafe impl Send`/`Sync`. The `unsafe` keyword is the author promising:
//! "I have verified the invariants the compiler could not". Getting this wrong is
//! a classic source of data races (which are UB in Rust).
//!
//! - `Send`: it is sound to **move** the value to another thread.
//! - `Sync`: it is sound to share `&T` across threads (equivalently, `&T: Send`).
//!
//! See the existing repo demo `tests/thread_safety_by_hand.rs` for a related
//! `unsafe impl Send` used to ship a non-`Send` value across a thread boundary.

use std::ptr::NonNull;

/// A minimal owning, heap-allocated box built on a raw pointer.
///
/// Because it stores a `NonNull<T>` (a raw pointer), the compiler makes it
/// `!Send + !Sync` by default. But semantically it behaves exactly like
/// `Box<T>`: it *uniquely owns* the pointee. Therefore the standard `Box<T>`
/// reasoning applies, and we may re-assert the auto traits by hand.
struct MyBox<T> {
    ptr: NonNull<T>,
}

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        // `Box::into_raw` gives us a uniquely-owned, non-null, aligned pointer.
        let raw = Box::into_raw(Box::new(value));
        // SAFETY: `Box::into_raw` never returns null.
        let ptr = unsafe { NonNull::new_unchecked(raw) };
        MyBox { ptr }
    }

    fn get(&self) -> &T {
        // SAFETY: we uniquely own the allocation for as long as `self` lives,
        // so producing a shared reference tied to `&self` is sound.
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        // SAFETY: `ptr` was produced by `Box::into_raw` and is owned uniquely by
        // `self`; reconstructing the `Box` exactly once frees it correctly.
        unsafe {
            drop(Box::from_raw(self.ptr.as_ptr()));
        }
    }
}

// SAFETY (Send): `MyBox<T>` uniquely owns its `T` (like `Box<T>`). Moving the
// box to another thread moves sole ownership; no aliasing can occur. This is
// sound exactly when `T: Send`.
unsafe impl<T: Send> Send for MyBox<T> {}

// SAFETY (Sync): a shared `&MyBox<T>` only hands out `&T`. Sharing it across
// threads is sound exactly when `&T` can be shared, i.e. when `T: Sync`.
unsafe impl<T: Sync> Sync for MyBox<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn behaves_like_a_box() {
        let b = MyBox::new(String::from("hello"));
        assert_eq!(b.get(), "hello");
    }

    #[test]
    fn can_be_sent_to_another_thread() {
        let b = MyBox::new(1234_u64);

        // This only compiles because of our `unsafe impl Send`.
        let handle = std::thread::spawn(move || *b.get());

        assert_eq!(handle.join().unwrap(), 1234);
    }
}
