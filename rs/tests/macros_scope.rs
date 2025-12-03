// Rust 2018+ : export de macros
mod macros {
    #[macro_export] // now available outside of this module as crate::my_macro!()
    macro_rules! my_macro {
        () => {
            println!("Hello")
        };
    }
}

fn main() {
    my_macro!();
}

#[test]
fn test() {
    main()
}
