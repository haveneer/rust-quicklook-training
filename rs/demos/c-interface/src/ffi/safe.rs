//! Safe wrappers around the raw bindings in `manual.rs`. This is where the
//! `unsafe` boundary actually lives — callers of this module never write
//! `unsafe` themselves.

use std::os::raw::{c_int, c_void};

use super::manual::{self, CCounter, Point};

/// Case 1: takes the struct by value, then passes its address to C.
pub fn norm(p: Point) -> f64 {
    unsafe { manual::point_norm(&p as *const Point) }
}

/// Case 2: slice -> (pointer, length), the idiom every C numeric API expects.
pub fn sum(values: &[f64]) -> f64 {
    unsafe { manual::sum_array(values.as_ptr(), values.len()) }
}

/// Case 3: RAII wrapper around the opaque `Counter*` handle. `Drop` is what
/// turns "C-side ownership" into "Rust-side ownership" — the caller never
/// has to remember to call `counter_free`.
pub struct Counter {
    ptr: *mut CCounter,
}

impl Counter {
    pub fn new(start: i32) -> Self {
        Counter {
            ptr: unsafe { manual::counter_new(start as c_int) },
        }
    }

    pub fn inc(&mut self) {
        unsafe { manual::counter_inc(self.ptr) };
    }

    pub fn get(&self) -> i32 {
        unsafe { manual::counter_get(self.ptr) }
    }
}

impl Drop for Counter {
    fn drop(&mut self) {
        unsafe { manual::counter_free(self.ptr) };
    }
}

/// Case 4 (bonus): lets C call back into an arbitrary Rust `FnMut(i32)`.
/// `extern "C" fn trampoline` is the fixed C-callable shim; `user_data`
/// carries the actual closure across the boundary as a raw pointer, and the
/// trampoline casts it back before calling it. This pattern (monomorphized
/// trampoline + user_data pointer) is how most C callback APIs get wrapped
/// safely in Rust.
pub fn for_each<F: FnMut(i32)>(values: &[i32], mut f: F) {
    extern "C" fn trampoline<F: FnMut(i32)>(value: c_int, user_data: *mut c_void) {
        let closure: &mut F = unsafe { &mut *(user_data as *mut F) };
        closure(value);
    }

    unsafe {
        manual::for_each(
            values.as_ptr(),
            values.len(),
            trampoline::<F>,
            &mut f as *mut F as *mut c_void,
        );
    }
}
