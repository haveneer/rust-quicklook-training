#[test]
fn flipflap() {
    let x = (1, 2);
    let y = x;
    let (a, b) = x;
    let (mut a, mut b) = (2, 1);
    // (a, b) = (3, 2); // error[E0658]: destructuring assignments are unstable
}