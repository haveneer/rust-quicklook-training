#[cfg(test)]
mod tests1 {
    type Charlie = Vec<u8>;

    struct A {
        #[allow(dead_code)]
        charlie: Charlie,
    }

    struct B<'a> {
        charlie: &'a A,
    }

    #[allow(dead_code)]
    struct C<'a> {
        a: A,
        b: B<'a>,
    }

    #[test]
    fn test() {
        let charlie = Charlie::new();
        let a = A { charlie };
        let b = B { charlie: &a };
        // let c = C { a, b }; // forbidden
        //                    // there is no way to build C if b referenced the same embedded a
        let f = move || {
            // let x = a;  // you can use ONLY ONE
            let x = b; // but not both
            x.charlie;
        };
        f();
    }
}

#[cfg(test)]
mod tests2 {
    use std::rc::Rc;

    type Charlie = Vec<u8>;

    struct A {
        charlie: Charlie,
    }

    struct B {
        charlie: Rc<A>,
    }

    #[allow(dead_code)]
    struct C {
        a: Rc<A>,
        b: B,
    }

    #[test]
    fn test() {
        let charlie = Charlie::new();
        let a = Rc::new(A { charlie });
        let b = B { charlie: a.clone() };

        // Both c and f solutions are OK but not in the same time; b is still moved
        // let c = C { a: a.clone(), b }; // OK

        let f = move || {
            let xa = a; // both OK
            let xb = b;
            &xa.charlie; // ref required since Charlie does not implement the `Copy` trait
            xb.charlie;
        };
        f();
    }
}
