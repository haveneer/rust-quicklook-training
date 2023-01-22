#[cfg(test)]
mod tests {
    trait Moon {
        fn moon(&self); // no default behaviour
    }

    struct A {}

    struct B {}

    #[rustfmt::skip]
    impl Moon for A {
        fn moon(&self) { /* ... */ }
    }

    #[rustfmt::skip]
    impl Moon for B {
        fn moon(&self) { /* ... */ }
    }

    #[rustfmt::skip]
    impl Moon for f64 {
        fn moon(&self) { /* ... */ }
    }

    #[test]
    fn main() {
        let a = A {};
        let b = B {};

        a.moon();
        A::moon(&a);

        b.moon();
        B::moon(&b);

        3.14.moon();
        f64::moon(&3.14);
    }
}
