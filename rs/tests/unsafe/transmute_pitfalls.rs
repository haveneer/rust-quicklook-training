//! `transmute`, layout and validity — framed as *authoring* hazards.
//!
//! `std::mem::transmute::<A, B>(x)` reinterprets the bytes of an `A` as a `B`.
//! It is the single most dangerous function in the language. The compiler checks
//! exactly **one** thing for you: that `size_of::<A>() == size_of::<B>()`. A size
//! mismatch is a *compile error* (see the matching trybuild case
//! `failures/*/transmute_size_mismatch.rs`).
//!
//! Everything else is on the author:
//! - **validity**: the bytes must be a *valid* `B`. Transmuting `3u8` into a
//!   `bool` is UB (only `0`/`1` are valid `bool`s), even though the sizes match.
//! - **layout**: `repr(Rust)` field order/padding is unspecified, so transmuting
//!   between two structs is UB unless you control layout with `#[repr(C)]` /
//!   `#[repr(transparent)]`.
//! - **lifetimes / provenance**: `transmute` can silently extend a lifetime or
//!   fabricate a `&mut`, which is UB. (This is *also* the shape behind the
//!   `cve-rs` soundness hole — see `cve_rs_lifetime_expansion.rs`.)
//!
//! Rule of thumb: reach for a *safer alternative* first. Most real uses of
//! `transmute` can be replaced by `as` casts, `f32::to_bits`/`from_bits`,
//! `u32::from_ne_bytes`, `slice::from_raw_parts`, or pointer casts.

#[cfg(test)]
mod tests {
    #[test]
    fn safer_alternatives_to_transmute() {
        // Float <-> bits: use the dedicated, *safe* methods instead of
        // `transmute::<f32, u32>`.
        let bits = 1.0_f32.to_bits();
        assert_eq!(bits, 0x3F80_0000);
        assert_eq!(f32::from_bits(bits), 1.0);

        // Integer <-> bytes: `to_ne_bytes` / `from_ne_bytes` are safe and make
        // endianness explicit.
        let bytes = 0x1234_5678_u32.to_ne_bytes();
        assert_eq!(u32::from_ne_bytes(bytes), 0x1234_5678);

        // Numeric reinterpretation that *is* a value conversion: use `as`.
        let signed = -1_i8;
        assert_eq!(signed as u8, 0xFF);
    }

    #[test]
    // Clippy even nudges us toward `to_ne_bytes()` here (lint
    // `transmute_num_to_bytes`); we keep the `transmute` only to *show* a sound
    // use, and silence the lint locally to make that intent explicit.
    #[allow(clippy::transmute_num_to_bytes)]
    fn transmute_when_it_is_actually_sound() {
        // A sound use: `u32` and `[u8; 4]` have the same size, and *every* byte
        // pattern is valid for both, so this reinterpretation can never observe
        // an invalid value. (Still, `to_ne_bytes` above is the idiomatic way.)
        let value = 0xDEAD_BEEF_u32;

        // SAFETY: size_of::<u32>() == size_of::<[u8; 4]>() and all bit patterns
        // are valid for `[u8; 4]`, so the result is always a valid value.
        let as_bytes: [u8; 4] = unsafe { std::mem::transmute(value) };

        assert_eq!(as_bytes, value.to_ne_bytes());
    }

    // DOCUMENTED-ONLY validity hazard (intentionally NOT executed):
    //
    //     let b: bool = unsafe { std::mem::transmute(3_u8) }; // UB!
    //
    // The sizes match (`bool` is 1 byte), so this *compiles*, but `3` is not a
    // valid `bool` (only `0` and `1` are). The result is an invalid value and
    // any use of it is Undefined Behavior. Running it under Miri reports an
    // "invalid value" error; on real hardware it may silently misbehave. This
    // is exactly why "it compiled and seemed to work" is never a soundness
    // argument for `transmute`.
}
