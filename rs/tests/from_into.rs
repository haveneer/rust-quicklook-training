use std::convert::From;

struct Foo {
    str: &'static str,
    n: u16,
}

impl From<&'static str> for Foo {
    fn from(v: &'static str) -> Self {
        Foo { str: v, n: 0 }
    }
}

impl From<(&'static str, u16)> for Foo {
    fn from(v: (&'static str, u16)) -> Self {
        Foo { str: v.0, n: v.1 }
    }
}

impl Foo {
    pub fn new<T>(v: T) -> Foo
    where
        T: Into<Foo> + std::fmt::Debug, // Into trait constraint
        Foo: From<T>,
    {
        println!("{:?}", v);
        // let result = v.into();  // equivalent
        let result = Foo::from(v); //    forms
        result
    }
}

fn main() {
    let _f = Foo::from("Bob");
    let _f = Foo::from(("Mary", 16));
    let _f = Foo::new("Bob");
    let _f = Foo::new(("Mary", 16));
    let _f: Foo = ("Mary", 16).into();
}

#[test]
fn basic_tests() {
    main()
}
