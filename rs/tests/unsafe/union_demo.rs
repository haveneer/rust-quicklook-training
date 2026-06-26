//! `union` field access: the author owns the "which variant is active?" invariant.
//!
//! Unlike an `enum`, a `union` has **no tag** telling you which field currently
//! holds a valid value — all fields share the same storage. Writing a field is
//! safe, but **reading** a field is `unsafe`, because the compiler cannot know
//! whether the bytes you are about to interpret are a valid value of that
//! field's type.
//!
//! Author's obligation: only read the field that was last written (or another
//! field whose type can validly reinterpret those exact bytes — i.e. an
//! intentional `transmute`-like reinterpretation, with all the validity caveats
//! that implies). Reading an *invalid* bit pattern for the field's type (e.g. a
//! `bool` that is neither 0 nor 1) is Undefined Behavior.

/// Reinterprets the storage of a 32-bit float and a 32-bit unsigned integer.
union F32Bits {
    f: f32,
    bits: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_the_field_you_wrote() {
        let u = F32Bits { f: 1.0_f32 };

        // SAFETY: we wrote the `f` field and read it back; the active field
        // matches the one we read, so the value is valid.
        let same = unsafe { u.f };
        assert_eq!(same, 1.0);
    }

    #[test]
    fn intentional_bit_reinterpretation() {
        // Writing `f`, then reading `bits`, is a deliberate type-pun. It is
        // sound here because *every* 32-bit pattern is a valid `u32`, so reading
        // `bits` can never observe an invalid value.
        let u = F32Bits { f: 1.0_f32 };

        // SAFETY: any bit pattern is a valid `u32`; reinterpreting the float's
        // bytes as `u32` cannot produce an invalid value.
        let raw = unsafe { u.bits };

        // IEEE-754: 1.0_f32 has the bit pattern 0x3F80_0000.
        assert_eq!(raw, 0x3F80_0000);
        assert_eq!(f32::from_bits(raw), 1.0);

        // CAUTION: the reverse direction is NOT always sound. Writing arbitrary
        // `bits` and then reading a field whose type has *invalid* bit patterns
        // (e.g. `bool`, `char`, an enum, a reference) would be UB if those bytes
        // are not a valid value of that type. `u32` is special: it has no
        // invalid patterns.
    }
}
