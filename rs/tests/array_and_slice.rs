// More info: https://stackoverflow.com/questions/30794235/what-is-the-difference-between-a-slice-and-an-array

#[test]
fn array_and_slice() {
    // This is coercion:
    let values: [u32; 5] = [1, 2, 3, 4, 5];
    let slice: &[u32] = &values;

    let x: &[u8] = &[1, 2, 3];
    println!("{:?}", x);

    let y: &[u8; 3] = &[1, 2, 3];
    println!("{:?}", y);
}
