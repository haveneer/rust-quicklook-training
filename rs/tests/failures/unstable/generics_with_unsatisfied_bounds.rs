trait Stringable {
    fn stringify() -> String;
}

struct Cat {}

impl Cat {
    fn pet() {}
}

impl Stringable for Cat {
    fn stringify() -> String {
        "meow".to_string()
    }
}

fn f<T: Stringable>(a: T) {
    a.pet();  // error[E0599]: no method named `pet` found for type `T` in the current scope
}

fn main() {
    f(Cat {});
}

#[test]
fn test_main() { main(); }