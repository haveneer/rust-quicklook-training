macro_rules! define_enum {
    ($name:ident { $($variant:ident),* }) => {
        enum $name {
            $($variant),*
        }

        impl $name {
            fn variants() -> &'static [&'static str] {
                &[$(stringify!($variant)),*]
            }

            fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
        }
    };
}

define_enum!(Color { Red, Green, Blue });

fn main() {
    println!("{:?}", Color::variants());
    // ["Red", "Green", "Blue"]
    println!("{}", Color::Red.name());
    // "Red"
}

#[test]
fn test() {
    main()
}
