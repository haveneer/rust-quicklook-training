//! Hand-written FFI bindings, matching `c/clib.h` field-for-field.
//!
//! This is the "no external crate" way of doing C interop: every signature
//! below has to be kept in sync with the header by hand. Fine for a small,
//! stable API like this one; for a real-world C library with hundreds of
//! functions, `bindgen` (see `generated.rs`, behind the `bindgen-compare`
//! feature) generates the same code automatically from the header instead.

use std::os::raw::{c_int, c_void};

/// Case 1: a plain-old-data struct shared with C. `#[repr(C)]` is what makes
/// the field layout match the C struct byte-for-byte — without it, rustc is
/// free to reorder fields and this would be undefined behavior.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Case 3: an opaque handle. Rust never looks inside `Counter` — the real
/// fields are private to `clib.c` — it only ever holds a `*mut CCounter`.
/// The zero-sized `_private` field with no public constructor is the
/// standard trick to make this type impossible to instantiate or read from
/// on the Rust side.
#[repr(C)]
pub struct CCounter {
    _private: [u8; 0],
}

pub type ForEachCallback = extern "C" fn(value: c_int, user_data: *mut c_void);

unsafe extern "C" {
    pub fn point_norm(p: *const Point) -> f64;
    pub fn sum_array(data: *const f64, len: usize) -> f64;

    pub fn counter_new(start: c_int) -> *mut CCounter;
    pub fn counter_inc(c: *mut CCounter);
    pub fn counter_get(c: *const CCounter) -> c_int;
    pub fn counter_free(c: *mut CCounter);

    pub fn for_each(arr: *const c_int, len: usize, cb: ForEachCallback, user_data: *mut c_void);
}
