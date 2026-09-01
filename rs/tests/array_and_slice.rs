#[test]
fn array_and_slice() {
    // [T; N] est alloué "on stack" par défaut (taille fixe connue à la compilation),
    // contrairement à Vec<T> qui alloue son buffer sur le tas.
    let values: [u32; 5] = [1, 2, 3, 4, 5];
    assert_eq!(
        std::mem::size_of_val(&values),
        5 * std::mem::size_of::<u32>()
    );

    // This is coercion:
    let slice: &[u32] = &values;

    let x: &[u8] = &[1, 2, 3];
    println!("{:?}", x);

    let y: &[u8; 3] = &[1, 2, 3];
    println!("{:?}", y);

    // Indexation et slicing (même syntaxe pour array et slice):
    assert_eq!(values[0], 1);
    assert_eq!(&values[1..3], &[2, 3]);
    assert_eq!(&slice[1..3], &[2, 3]);
}
