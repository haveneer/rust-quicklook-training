// Example with conversion
// Implement From => Into implicitly
// https://doc.rust-lang.org/std/convert/trait.From.html
// Show how Into trait is implemented in std

fn is_hello<T: Into<Vec<u8>>>(s: T) {
    let bytes = b"hello".to_vec();
    assert_eq!(bytes, s.into());
}

#[test]
fn main() {
    let s = "hello".to_string();
    is_hello(s);
}