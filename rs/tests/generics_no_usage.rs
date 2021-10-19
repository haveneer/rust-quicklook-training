fn foo<T>(arg: T) {}

fn main() {
    foo(1);
    foo(3.14);
    foo("Hello");
}

#[test]
fn test_foo() { main() }
