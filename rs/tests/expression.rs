pub trait AnyExt {
    fn type_name(&self) -> &'static str;
}

impl<T> AnyExt for T {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

fn main() {
    {
        // without semicolon
        let x = { 1 };
        println!("type of {{ 1 }} is {}", x.type_name());
        assert_eq!(x.type_name(), "i32");
    }

    {
        #[rustfmt::skip]
        // with semicolon
        let x = { 1; };
        println!("type of {{ 1; }} is {}", x.type_name());
        assert_eq!(x.type_name(), "()");
    }

    {
        let test = true;
        // without semicolon
        let x = if test { "Hello" } else { "World" };
        println!("type of x is {}", x.type_name());
        assert_eq!(x.type_name(), "&str");
    }

    {
        let test = true;
        #[rustfmt::skip]
        // with semicolon
        let y = if test { "Hello"; } else { "World"; };
        println!("type of y is {}", y.type_name());
        assert_eq!(y.type_name(), "()");
    }
}

#[test]
fn test() {
    main();
}
