macro_rules! create_function {
    // Pattern : identifier
    ($func_name:ident) => {
        fn $func_name() {
            println!("Function {:?} called", stringify!($func_name));
        }
    };
}

// Utilisation
create_function!(foo);
create_function!(bar);

fn main() {
    foo(); // "Function "foo" called"
    bar(); // "Function "bar" called"
}

#[test]
fn test() {
    main()
}
