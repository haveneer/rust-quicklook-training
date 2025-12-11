#[test]
fn flipflap() {
    let x = (1, 2);
    let y = x;
    let (a, b) = x;
    let (a, b) = (b, a);
    // (a, b) = (3, 2); // error[E0658]: destructuring assignments are unstable
}
