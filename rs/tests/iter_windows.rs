#[test]
fn main() {
    let tst = String::from("abcdefg");
    let inter = tst.chars().collect::<Vec<char>>();
    let mut windows = inter.windows(3);

    // prints ['a', 'b', 'c']
    println!("{:?}", windows.next().unwrap());
    // prints ['b', 'c', 'd']
    println!("{:?}", windows.next().unwrap());
    // etc...
    println!("{:?}", windows.next().unwrap());
}