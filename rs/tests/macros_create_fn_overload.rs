macro_rules! create_fn {
    // Pattern 1 : pas d'arguments
    ($name:ident) => {
        fn $name() {
            println!("{}()", stringify!($name));
        }
    };
    // Pattern 2 : avec des arguments
    ($name:ident, $($arg:ident : $ty:ty),*) => {
        fn $name($($arg: $ty),*) {
            println!("{}({:?})", stringify!($name), ($($arg,)*));
        }
    };
}

create_fn!(foo);
create_fn!(bar, x: i32, y: i32);

fn main() {
    foo();
    bar(10, 20);
}

#[test]
fn test() {
    main()
}
