fn foo<T>(arg: T) -> T { return arg + 1; }

fn main() {
    foo(1);
    foo(3.14);
    foo("Hello");
}

#[test]
fn test_main() { main() }
