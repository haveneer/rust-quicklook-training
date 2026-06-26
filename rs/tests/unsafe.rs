//! Driver crate for the "writing `unsafe` Rust" section.
//!
//! This integration-test crate groups every demo about *authoring* `unsafe`
//! code (as opposed to merely *calling* an existing `unsafe` function). Each
//! topic lives in its own module under `tests/unsafe/`, and the progressive
//! walkthrough lives next to them in `tests/unsafe/UNSAFE_GUIDE.md`.
//!
//! Run everything on the pinned stable toolchain with:
//! ```text
//! cargo test --test unsafe -- --nocapture
//! ```
//!
//! Because `tests/unsafe.rs` is itself the crate root of this integration test,
//! `mod foo;` is resolved relative to `tests/` (the directory of the root file),
//! *not* `tests/unsafe/`. We therefore use explicit `#[path = "unsafe/<name>.rs"]`
//! to keep every demo grouped inside the dedicated `tests/unsafe/` subdirectory.

// Positive, runnable demos. Each one lives in `tests/unsafe/<name>.rs`.
#[path = "unsafe/cve_rs_lifetime_expansion.rs"]
mod cve_rs_lifetime_expansion;
#[path = "unsafe/ptr_basics.rs"]
mod ptr_basics;
#[path = "unsafe/safe_abstraction.rs"]
mod safe_abstraction;
#[path = "unsafe/static_mut_and_addr_of.rs"]
mod static_mut_and_addr_of;
#[path = "unsafe/transmute_pitfalls.rs"]
mod transmute_pitfalls;
#[path = "unsafe/union_demo.rs"]
mod union_demo;
#[path = "unsafe/unsafe_fn_contract.rs"]
mod unsafe_fn_contract;
#[path = "unsafe/unsafe_send_sync.rs"]
mod unsafe_send_sync;

#[cfg(test)]
mod tests {
    /// Compile-fail demos, mirroring the existing `*_failures/{stable,unstable}/`
    /// pattern (see `tests/ownership.rs`). The stable/unstable split absorbs the
    /// diagnostic wording differences between the pinned stable toolchain and
    /// nightly (where `build.rs` auto-enables the `nightly` feature).
    #[test]
    #[rustfmt::skip]
    fn unsafe_failures() {
        let t = trybuild::TestCases::new();

        let version_path = if cfg!(feature = "nightly") { "unstable" } else { "stable" };

        t.compile_fail(format!("tests/unsafe/failures/{version_path}/transmute_size_mismatch.rs"));
        t.compile_fail(format!("tests/unsafe/failures/{version_path}/send_not_implemented.rs"));
    }
}
