#[test]
fn main() {
    let mut v = Vec::new(); // here, v is not yet fully typed
    let mut var; // here, val is not yet fully typed

    // ... do things without using `var` and `v`

    v.push(3); // now v is a vector of i32
    var = 3.15; // now var is a f64

    // v.push(3.14); // error: expected integer, found floating-point number
    // var = 3; // error: expected floating-point number, found integer
}
