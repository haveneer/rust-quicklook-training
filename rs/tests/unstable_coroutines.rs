#![cfg_attr(feature = "nightly", feature(coroutines, coroutine_trait, stmt_expr_attributes))]
// cf https://doc.rust-lang.org/beta/unstable-book/language-features/generators.html

#[cfg(feature = "nightly")]
mod unstable_mods;

#[cfg(feature = "nightly")]
#[test]
fn test() {
    unstable_mods::coroutine_test::coroutine_test()
}
