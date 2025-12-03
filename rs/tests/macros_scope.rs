// Rust 2018+ : export de macros
mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Hello")
        };
    }
}

// Import avec use
use crate::my_macro;

fn main() {
    my_macro!();
}

#[test]
fn test() {
    main()
}
