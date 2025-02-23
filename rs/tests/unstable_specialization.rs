#![cfg_attr(feature = "nightly", feature(specialization))]

// https://github.com/rust-lang/rfcs/pull/1210

pub trait Compare<T: ?Sized = Self> {
    fn equal(&self, other: &T) -> bool;
}

#[cfg(feature = "nightly")]
mod nightly {
    // Default implementation (nightly only)
    impl<T> Compare<T> for T
    where
        Self: core::cmp::PartialEq<T>,
    {
        default fn equal(&self, other: &T) -> bool {
            print!("Compare default Self with T");
            self == other
        }
    }

    // Specialization for T=f32
    // TODO: what happens without this ?
    impl Compare<f32> for f32 {
        fn equal(&self, other: &f32) -> bool {
            print!("Compare f32 with f32");
            self == other
        }
    }

    // Specialization for T=i32
    impl Compare<i32> for f32 {
        fn equal(&self, other: &i32) -> bool {
            print!("Compare f32 with i32");
            &(*self as i32) == other
        }
    }

    struct Foo(i32);

    // Specialization for T=Foo(i32)
    impl Compare<i32> for Foo {
        fn equal(&self, other: &i32) -> bool {
            print!("Compare Foo with i32");
            &self.0 == other
        }
    }

    fn main() {
        println!(" = {}", 1.0f32.equal(&1.0f32));
        println!(" = {}", 1.0f32.equal(&2i32));
        println!(" = {}", Foo(1i32).equal(&2i32));
        println!(" = {}", 1i32.equal(&2i32));
    }

    #[test]
    fn test() {
        main();
    }
}
