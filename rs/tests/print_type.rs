fn print_type<T>(_t: &T) {
    println!("{}", std::any::type_name::<T>());
}


#[test]
fn test() {
    print_type(&1);
    let mut v = Vec::new();
    v.push(1); // without this line, compiler cannot infer type
    print_type(&v);
    v.push(1); // without this line, compiler cannot infer type
}