#[test]
fn test0() {
    let s1 = String::from("hello");
    let s2 = "world".to_string();
    let s3 = "folks";

    let s = s1+&s2+s3; // String + &String + &str
    println!("{}", s);
}

#[test]
fn test1() {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = String::from("folks");

    let s = [s1, s2, s3].concat();
    println!("{}", s);
}

#[test]
fn test2() {
    let s1 = "hello";
    let s2 = "world";
    let s3 = "folks";

    let s = [s1, s2, s3].concat();
    let s = [s1, s2, s3].join(" ");
    println!("{}", s);
}

#[test]
fn test3() {
    let s1 = "hello".to_string();
    let s2 = "world";
    let s3 = String::from("folks");

    let s = format!("{}{}{}", s1, s2, s3);
    println!("{}", s);
}

#[test]
fn test4() {
    let s = concat!("test", 10, 'b', true); // only for literals
    assert_eq!(s, "test10btrue");
}