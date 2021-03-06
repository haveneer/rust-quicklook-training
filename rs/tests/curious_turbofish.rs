#[test]
fn test() {
    let v = vec![1];

    let mut x = Vec::new(); // turbo fish + outer < >
    x.push(1);

    let mut x = Vec::<i32>::new(); // turbo fish without outer < >
    x.push(1);

    let mut x = <Vec<i32>>::new(); // no turbo fish but outer < >
    x.push(1);

    let x = <String>::new(); // <T>::method syntax (sometimes useful to remove ambiguities)
}
