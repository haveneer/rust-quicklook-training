// Integration tests are like out-of-the crate tests
// Very useful for libs
use mylib1::f4;
// use mylib1::f6b; // not visible out of crate

#[test]
fn test() {
    assert!(f4());
}
