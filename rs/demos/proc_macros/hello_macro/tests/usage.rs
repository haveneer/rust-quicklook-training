use hello_macro::HelloMacro;

trait HelloMacro {
    fn hello_macro();
}

#[derive(HelloMacro)]
struct Pancakes;

#[test]
fn test_hello_macro() {
    Pancakes::hello_macro();
    // "Hello from Pancakes!"
}
