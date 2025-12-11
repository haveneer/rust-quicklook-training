macro_rules! create_function {
    // Pattern : identifier
    ($func_name:ident) => {
        fn $func_name() {
            println!("Function {:?} called", stringify!($func_name));
        }
    };
}

// Utilisation; choisissez votre style 👔
create_function!(foo);
create_function![bar];
create_function! {baz} // ⚠️ item macro invocation declarations are not followed by a semicolon

fn main() {
    foo(); // "Function "foo" called"
    bar(); // "Function "bar" called"
    baz(); // "Function "baz" called"
}

#[test]
fn test() {
    main()
}
