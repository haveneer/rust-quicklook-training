//! The safe-code soundness boundary: `cve-rs` / `rust-lang/rust#25860`.
//!
//! Everything else in this directory is about code that is `unsafe` *by
//! construction*. This file is the opposite and far more unsettling lesson:
//! **`unsafe` is not the only place Undefined Behavior can come from.** The
//! functions below contain **no `unsafe` keyword at all**, yet they let safe
//! code fabricate a dangling reference and read freed memory.
//!
//! This is possible because of a long-standing soundness hole in the compiler's
//! handling of higher-ranked lifetimes / well-formedness:
//! <https://github.com/rust-lang/rust/issues/25860> (open since 2015). The
//! `cve-rs` project (<https://github.com/Speykious/cve-rs>) weaponizes exactly
//! this bug — notably its `src/lifetime_expansion.rs`, faithfully reproduced
//! below — to build buffer overflows, use-after-free and segfaults *without*
//! `unsafe`.
//!
//! ## How the trick works
//! `lifetime_translator` is, on its own, **sound**: its `_val_a: &'a &'b ()`
//! argument forces `'b: 'a` (a `&'a` to a `&'b` cannot outlive what it points
//! to), so returning the `&'b T` as `&'a T` is fine.
//!
//! The bug is in `expand`: by coercing `lifetime_translator` to a
//! `for<'x> fn(_, &'x T) -> &'b T` function pointer and feeding it the
//! `'static` `STATIC_UNIT`, the compiler *fails* to enforce the
//! well-formedness constraint, so an arbitrary input lifetime `'a` gets
//! "expanded" to any `'b` — including `'static`. That is enough to defeat the
//! borrow checker entirely.
//!
//! ## Why this file is gated
//! Actually *using* an expanded reference after its referent is dropped is
//! genuine UB. On real hardware that is nondeterministic: it may print garbage,
//! print the right thing by luck, or crash. To keep the default `cargo test`
//! run deterministic and green, the UB-triggering test is:
//! - `#[cfg(feature = "nightly")]` — it only exists on the nightly toolchain
//!   (where the repo's `build.rs` enables the `nightly` feature), and
//! - `#[ignore]` — even there it is not run by default.
//!
//! ## Observing the UB deterministically (Miri)
//! Miri interprets the program and turns latent UB into a clear, reproducible
//! diagnostic instead of a random crash. Run:
//! ```bash
//! cargo +nightly miri test --test unsafe -- --ignored cve_rs
//! ```
//! Miri reports a use-after-free ("pointer to <alloc> was dereferenced after
//! this allocation got freed") at the read of the dangling reference.

/// Converts lifetime `'b` to lifetime `'a`. **Sound on its own** (see module
/// docs): `_val_a: &'a &'b ()` requires `'b: 'a`, so `&'b T` is valid for `'a`.
#[inline(never)]
fn lifetime_translator<'a, 'b, T: ?Sized>(_val_a: &'a &'b (), val_b: &'b T) -> &'a T {
    val_b
}

/// A `()` reference with `'static` lifetime — the seed for the expansion.
const STATIC_UNIT: &&() = &&();

/// Expands an arbitrary input lifetime `'a` into *any* lifetime `'b`.
///
/// This is the unsound part — and it contains **no `unsafe`**. It exploits the
/// well-formedness hole of rust#25860 via the higher-ranked function-pointer
/// coercion below.
fn expand<'a, 'b, T: ?Sized>(x: &'a T) -> &'b T {
    // Exactly the `cve-rs` coercion: the first argument type is left to
    // inference (`_`), and the higher-ranked `for<'x>` binder is where the
    // well-formedness check is (wrongly) dropped.
    let f: for<'x> fn(_, &'x T) -> &'b T = lifetime_translator;
    f(STATIC_UNIT, x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expansion_compiles_and_is_benign_on_truly_static_input() {
        // Calling `expand` on an input that really *is* `'static` is harmless:
        // no reference outlives its data here. This only demonstrates that the
        // unsound function compiles and runs on stable — which is the whole
        // problem: nothing flags it.
        let s: &'static str = "I am genuinely static";
        let widened: &'static str = expand(s);
        assert_eq!(widened, "I am genuinely static");
    }

    /// Forges a `&'static String` that actually points to a dropped local —
    /// a use-after-free reachable from 100% safe code.
    ///
    /// Gated + ignored so it never runs in the normal suite. Inspect it with:
    /// `cargo +nightly miri test --test unsafe -- --ignored cve_rs`.
    #[cfg(feature = "nightly")]
    #[test]
    #[ignore = "triggers real UB (use-after-free); run only under Miri"]
    fn cve_rs_use_after_free_from_safe_code() {
        fn dangle() -> &'static String {
            let local = String::from("freed before it is read");
            // `&local` is `&'a String`; `expand` unsoundly widens it to
            // `&'static String`, escaping the borrow checker. NO `unsafe` here.
            let escaped: &'static String = expand(&local);
            escaped
            // `local` is dropped here; `escaped` now dangles.
        }

        let dangling = dangle();
        // Reading through the dangling reference is the use-after-free. Miri
        // reports it deterministically; on hardware it is UB.
        println!("read after free: {dangling}");
    }
}
