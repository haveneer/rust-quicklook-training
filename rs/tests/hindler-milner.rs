#[test]
fn main() {
    let mut v = Vec::new();
    let mut var;
    // do things without using `var`
    v.push(3);
    var = 3.14;

    // v.push(3.14); // error: expected integer, found floating-point number
    // var = 3; // error: expected floating-point number, found integer
}
