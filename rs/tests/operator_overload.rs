struct A;

struct B;

impl std::ops::Add<A> for A {
    type Output = ();
    fn add(self, rhs: A) -> Self::Output {}
}

impl std::ops::Add<B> for A {
    type Output = ();
    fn add(self, rhs: B) -> Self::Output {}
}

// You should implement trait for all combinaison
// - &T op U
// - T op &U
// - &T op &U
// - T op U
// To do so, either use rust internals https://stackoverflow.com/a/38815035/12430075
// or crate [impl_ops](https://docs.rs/impl_ops/0.1.1/impl_ops/index.html)

fn main() {
    A + A;
    A + B;
}

#[test]
fn test() {
    main();
}
