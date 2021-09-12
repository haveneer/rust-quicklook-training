pub trait AnyExt {
    fn type_name(&self) -> &'static str;
}

impl<T> AnyExt for T {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

fn main() {
    { // without semicolon
        let x = { 1 };
        println!("type of {{ 1 }} is {}", x.type_name());
        assert_eq!(x.type_name(), "i32");
    }

    {  // with semicolon
        let x = { 1; };
        println!("type of {{ 1; }} is {}", x.type_name());
        assert_eq!(x.type_name(), "()");
    }
}

#[test]
fn test() { main(); }