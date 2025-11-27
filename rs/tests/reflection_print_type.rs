fn print_type<T>(_t: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    print_type(&1);
    let mut v = Vec::new();
    v.push(1); // without this line, compiler cannot infer type (Why ?)
    print_type(&v);
}

#[test]
fn test() {
    main()
}
