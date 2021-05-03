#[cfg(test)]
mod tests {
    trait F {
        fn f(&self) {}
    }

    struct A {}

    struct B {}

    impl F for A {
        fn f(&self) {}
    }

    impl F for B {
        fn f(&self) {}
    }
    
    impl F for f64 {
        fn f(&self) {}
    }

    #[test]
    fn main() {
        let a = A {};
        let b = B {};

        a.f();
        A::f(&a);
        
        b.f();
        B::f(&b);
        
        3.14.f();
        f64::f(&3.14);
    }
}