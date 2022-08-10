#![cfg_attr(feature = "nightly", feature(generators))]
#![cfg_attr(feature = "nightly", feature(generator_trait))]
// cf https://doc.rust-lang.org/beta/unstable-book/language-features/generators.html

#[cfg(feature = "nightly")]
mod unstable_mods;

#[cfg(feature = "nightly")]
#[test]
fn test() {
    unstable_mods::generator_test::generator_test()
}
